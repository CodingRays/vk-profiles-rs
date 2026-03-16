//! Example showing setup code using a profile and changing some features.

extern crate vk_profiles_rs;

use std::ops::BitOr;

use ash::vk;
use vk_profiles_rs::{profiles, vp, VulkanProfiles};

fn main() {
    // use vulkan 1.2
    let profile = profiles::LunargMinimumRequirements1_2::profile_properties();

    let entry = ash::Entry::linked();
    let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();

    let instance =
        create_instance(&entry, &vk_profiles, &profile).expect("Failed to create instance");
    let (physical_device, device, queue_family_index, queue) =
        create_device(&vk_profiles, &instance, &profile)
            .inspect_err(|_err| {
                // Make sure we clean up our instance if we get an error
                unsafe {
                    instance.destroy_instance(None);
                }
            })
            .expect("Failed to create device");

    // We now have a working device using the selected profile.
    println!(
        "Created device for profile {:?} using queue family: {:?}",
        profile, queue_family_index
    );

    let buffer_size = 64;
    let buffer = create_storage_buffer_with_queryable_address(&device, buffer_size);
    println!("Created buffer with size {}", buffer_size);

    let (memory, mem_size, mem_type_index) =
        allocate_device_memory_for_address_buffer(&instance, physical_device, &device, buffer);
    println!(
        "Allocated memory with size {} in memory index {}",
        mem_size, mem_type_index
    );

    unsafe {
        // offset is 0 so there are no buffer offset requirement problems
        device.bind_buffer_memory(buffer, memory, 0)
    }
    .expect("Failed to bind buffer to memory");
    println!("Bound buffer to memory");

    let address_info = vk::BufferDeviceAddressInfo {
        buffer,
        ..Default::default()
    };
    let address = unsafe { device.get_buffer_device_address(&address_info) };
    println!("Buffer address: 0x{:x}", address);

    unsafe { device.queue_wait_idle(queue) }.unwrap();
    unsafe { device.device_wait_idle() }.unwrap();

    unsafe {
        device.destroy_buffer(buffer, None);
        device.free_memory(memory, None);
    }

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
    if !unsafe { vk_profiles.get_instance_profile_support(None, profile_properties)? } {
        panic!(
            "Profile {:?} is not supported for instance creation.",
            profile_properties
        );
    }

    let instance_info = vk::InstanceCreateInfo::default();
    let vp_instance_info = vp::InstanceCreateInfo {
        p_create_info: &instance_info,
        p_enabled_full_profiles: profile_properties,
        enabled_full_profile_count: 1,
        ..Default::default()
    };

    unsafe { vk_profiles.create_instance(entry, &vp_instance_info, None) }
}

/// Creates a device for the specified profile and creates a single queue supporting graphics operations.
///
/// We also enable the buffer_device_address feature which is not included by the LunarG minimum requirements profile for Vulkan 1.2.
fn create_device(
    vk_profiles: &VulkanProfiles,
    instance: &ash::Instance,
    profile_properties: &vp::ProfileProperties,
) -> Result<(vk::PhysicalDevice, ash::Device, u32, vk::Queue), vk::Result> {
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };

    for physical_device in physical_devices {
        // We select the first device supporting the profile
        if unsafe {
            vk_profiles.get_physical_device_profile_support(
                instance,
                physical_device,
                profile_properties,
            )?
        } {
            // Need to make sure the device supports buffer_device_address
            let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut features12);

            unsafe { instance.get_physical_device_features2(physical_device, &mut features) };

            if features12.buffer_device_address == vk::FALSE {
                // Device does not support buffer_device_address
                continue;
            }

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

            let queue_priorities = [1f32];
            let queue_info = vk::DeviceQueueCreateInfo {
                queue_family_index: 0,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
                ..Default::default()
            };

            // enable the additional features you need
            // these will get merged together with the features required by the profiles
            let mut to_enable_features = vk::PhysicalDeviceVulkan12Features {
                buffer_device_address: vk::TRUE,
                ..Default::default()
            };
            let device_info = vk::DeviceCreateInfo {
                p_queue_create_infos: &queue_info,
                queue_create_info_count: 1,
                ..Default::default()
            }
            .push_next(&mut to_enable_features);

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

            return Ok((physical_device, device, queue_family_index, queue));
        }
    }

    panic!(
        "No device supporting profile {:?} and buffer_device_address found.",
        profile_properties
    );
}

fn create_storage_buffer_with_queryable_address(
    device: &ash::Device,
    buffer_size: vk::DeviceSize,
) -> vk::Buffer {
    let buffer_info = vk::BufferCreateInfo {
        flags: vk::BufferCreateFlags::empty(),
        size: buffer_size,
        // enable SHADER_DEVICE_ADDRESS so we can query the buffer address
        usage: vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
            .bitor(vk::BufferUsageFlags::STORAGE_BUFFER),
        ..Default::default()
    };
    unsafe { device.create_buffer(&buffer_info, None) }
        .expect("Failed to create a buffer with the specific requirements")
}

/// Find the index of the first memory type that supports the buffer and the required property flags
fn find_memory_type(
    device_mem_props: &vk::PhysicalDeviceMemoryProperties,
    supported_memory_types_bitmask: u32,
    required_properties: vk::MemoryPropertyFlags,
) -> Option<u32> {
    for (i, memory_type) in device_mem_props.memory_types.iter().enumerate() {
        let mem_type_supported = supported_memory_types_bitmask & (1 << i) > 0;
        if mem_type_supported && memory_type.property_flags.contains(required_properties) {
            return Some(i as u32);
        }
    }
    None
}

fn allocate_device_memory_for_address_buffer(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    buffer: vk::Buffer,
) -> (vk::DeviceMemory, vk::DeviceSize, u32) {
    let buf_mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let mem_size = buf_mem_requirements.size;

    let device_mem_props =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let mem_type_index = find_memory_type(
        &device_mem_props,
        buf_mem_requirements.memory_type_bits,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .expect("Failed to find suitable memory type for the required buffer");

    let mut mem_allocate_flags = vk::MemoryAllocateFlagsInfo {
        flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS, // required for buffer
        ..Default::default()
    };
    let memory_info = vk::MemoryAllocateInfo {
        allocation_size: mem_size,
        memory_type_index: mem_type_index as u32,
        ..Default::default()
    }
    .push_next(&mut mem_allocate_flags);

    let memory =
        unsafe { device.allocate_memory(&memory_info, None) }.expect("Failed to allocate memory");

    (memory, mem_size, mem_type_index)
}
