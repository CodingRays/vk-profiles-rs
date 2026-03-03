//! Example showing setup code using a profile without any additional configuration.

use std::ffi::c_char;

use ash::vk;
use vk_profiles_rs::{profiles, vp, VulkanProfiles};

fn main() {
    let profile = profiles::LunargDesktopBaseline2024::profile_properties();

    let entry = ash::Entry::linked();
    let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();

    let instance =
        create_instance(&entry, &vk_profiles, &profile).expect("Failed to create instance");
    let (device, queue_family_index, queue) = create_device(&vk_profiles, &instance, &profile)
        .map_err(|err| {
            // Make sure we clean up our instance if we get an error
            unsafe {
                instance.destroy_instance(None);
            }
            err
        })
        .expect("Failed to create device");

    // We now have a working device using the selected profile.
    println!(
        "Created device for profile {:?} using queue family: {:?}",
        profile, queue_family_index
    );
    unsafe { device.queue_wait_idle(queue) }.unwrap();
    unsafe { device.device_wait_idle() }.unwrap();

    unsafe {
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}

/// Creates a instance for the specified profile
fn create_instance(
    entry: &ash::Entry,
    vk_profiles: &VulkanProfiles,
    profile_properties: &vp::ProfileProperties,
) -> Result<ash::Instance, vk::Result> {
    if !unsafe { vk_profiles.get_instance_profile_support(None, &profile_properties)? } {
        panic!(
            "Profile {:?} is not supported for instance creation.",
            profile_properties
        );
    }

    // LunargDesktopBaseline2024 has VK_KHR_swapchain as a device extension, which requires VK_KHR_SURFACE
    let extensions: Vec<*const c_char> = vec![ash::khr::surface::NAME.as_ptr()];

    let instance_info = vk::InstanceCreateInfo::default().enabled_extension_names(&extensions);
    let vp_instance_info = vp::InstanceCreateInfo {
        p_enabled_full_profiles: profile_properties,
        enabled_full_profile_count: 1,
        p_create_info: &instance_info,
        ..Default::default()
    };

    // vulkan_profiles will activate both manual and profile extensions
    unsafe { vk_profiles.create_instance(entry, &vp_instance_info, None) }
}

/// Creates a device for the specified profile and creates a single queue supporting graphics operations.
fn create_device(
    vk_profiles: &VulkanProfiles,
    instance: &ash::Instance,
    profile_properties: &vp::ProfileProperties,
) -> Result<(ash::Device, u32, vk::Queue), vk::Result> {
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };

    for physical_device in physical_devices {
        // We select the first device supporting the profile
        if unsafe {
            vk_profiles.get_physical_device_profile_support(
                instance,
                physical_device,
                &profile_properties,
            )?
        } {
            // Find the graphics queue
            let queues =
                unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
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

            let queue_priorities: [f32; 1] = [1.0];
            let queue_info = vk::DeviceQueueCreateInfo {
                queue_family_index: 0,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
                ..Default::default()
            };

            let device_info = vk::DeviceCreateInfo {
                p_queue_create_infos: &queue_info,
                queue_create_info_count: 1,
                ..Default::default()
            };

            let vp_device_info = vp::DeviceCreateInfo {
                p_create_info: &device_info,
                p_enabled_full_profiles: profile_properties,
                enabled_full_profile_count: 1,
                ..Default::default()
            };

            let device = unsafe {
                vk_profiles.create_device(instance, physical_device, &vp_device_info, None)?
            };

            let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

            return Ok((device, queue_family_index, queue));
        }
    }

    panic!(
        "No device supporting profile {:?} found.",
        profile_properties
    );
}
