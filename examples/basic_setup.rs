//! Example showing setup code using a profile without any additional configuration.

extern crate vk_profiles_rs;

use ash::vk;
use vk_profiles_rs::{VulkanProfiles, vp};

fn main() {
    let profile = vp::LunargDesktopPortability2021::profile_properties();

    let entry = ash::Entry::linked();
    let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();

    let instance = create_instance(&entry, &vk_profiles, &profile).expect("Failed to create instance");
    let (device, queue_family_index, queue) = create_device(&vk_profiles, &instance, &profile).map_err(|err| {
        // Make sure we clean up our instance if we get an error
        unsafe { instance.destroy_instance(None); }
        err
    }).expect("Failed to create device");

    // We now have a working device using the selected profile.
    println!("Created device for profile {:?} using queue family: {:?}", profile, queue_family_index);
    unsafe { device.queue_wait_idle(queue) }.unwrap();
    unsafe { device.device_wait_idle() }.unwrap();

    unsafe {
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}

/// Creates a instance for the specified profile
fn create_instance(entry: &ash::Entry, vk_profiles: &VulkanProfiles, profile: &vp::ProfileProperties) -> Result<ash::Instance, vk::Result> {
    if !unsafe { vk_profiles.get_instance_profile_support(None, &profile)? } {
        panic!("Profile {:?} is not supported for instance creation.", profile);
    }

    let instance_info = vk::InstanceCreateInfo::builder();
    let vp_instance_info = vp::InstanceCreateInfo::builder()
        .create_info(&instance_info)
        .profile(&profile);

    unsafe {
        vk_profiles.create_instance(entry, &vp_instance_info, None)
    }
}

/// Creates a device for the specified profile and creates a single queue supporting graphics operations.
fn create_device(vk_profiles: &VulkanProfiles, instance: &ash::Instance, profile: &vp::ProfileProperties) -> Result<(ash::Device, u32, vk::Queue), vk::Result> {
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };

    for physical_device in physical_devices {
        // We select the first device supporting the profile
        if unsafe { vk_profiles.get_physical_device_profile_support(instance, physical_device, profile)? } {
            // Find the graphics queue
            let queues = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
            let mut queue_family_index = None;
            for (queue_index, queue) in queues.iter().enumerate() {
                if queue.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    queue_family_index = Some(queue_index as u32);
                    break;
                }
            }
            if queue_family_index.is_none() {
                continue;
            }
            let queue_family_index = queue_family_index.unwrap();

            let queue_priority = 1f32;
            let queue_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(0)
                .queue_priorities(std::slice::from_ref(&queue_priority));

            let device_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(std::slice::from_ref(&queue_info));

            let vp_device_info = vp::DeviceCreateInfo::builder()
                .create_info(&device_info)
                .profile(&profile);

            let device = unsafe { vk_profiles.create_device(instance, physical_device, &vp_device_info, None)? };

            let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

            return Ok((device, queue_family_index, queue));
        }
    }

    panic!("No device supporting profile {:?} found.", profile);
}