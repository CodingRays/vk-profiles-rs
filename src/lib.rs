//! # Vulkan Profiles Library
//! <https://github.com/KhronosGroup/Vulkan-Profiles>
//! 
//! ## Examples
//! ```
//! use ash::vk;
//! use vk_profiles_rs::vp;
//! 
//! # fn main() -> ash::prelude::VkResult<()> {
//! let profile = vp::LunargDesktopPortability2021::profile_properties();
//! assert!(unsafe { vp::get_instance_profile_support(None, &profile)? });
//! 
//! let instance_info = vk::InstanceCreateInfo::builder();
//! 
//! let vp_instance_info = vp::InstanceCreateInfo::builder()
//!     .create_info(&instance_info)
//!     .profile(&profile);
//! 
//! let entry = ash::Entry::linked();
//! 
//! // The created instance is a standard [ash::Instance]
//! let instance = unsafe { vp::create_instance(&entry, &vp_instance_info, None)? };
//! # Ok(())
//! # }
//! ```
//! 
//! ## Ash design patterns
//! 
//! This crate uses [ash](https://github.com/ash-rs/ash) design patterns wherever possible to allow for seamless usage. Structs
//! have builder versions, enums use the same constructs etc.
//! 
//! ## Important notes
//! 
//! The vulkan loader must be statically linked in ash.

extern crate link_cplusplus;

mod prelude;
#[cfg(feature = "debug")]
pub mod enum_debugs;
pub mod vp;

#[cfg(test)]
mod tests {
    use crate::vp;
    use ash::vk;

    fn create_instance() -> (vp::ProfileProperties, ash::Instance) {
        let profile = vp::LunargDesktopPortability2021::profile_properties();
        assert!(unsafe { vp::get_instance_profile_support(None, &profile).unwrap() });

        let entry = ash::Entry::linked();

        let instance_info = vk::InstanceCreateInfo::builder();
        
        let vp_instance_info = vp::InstanceCreateInfo::builder()
            .create_info(&instance_info)
            .profile(&profile);

        let instance = unsafe { vp::create_instance(&entry, &vp_instance_info, None).unwrap() };

        (profile, instance)
    }

    #[test]
    fn test_enumerate_profiles() {
        let profiles = unsafe {
            vp::get_profiles().unwrap()
        };

        assert!(profiles.len() > 0);
        for profile in &profiles {
            println!("Profile {:?}: {:?}", unsafe { vp::get_instance_profile_support(None, profile).unwrap()}, profile);
        }

        unsafe { vp::get_profile_fallbacks(&profiles[0]).unwrap() };
    }

    #[test]
    fn test_enumerate_profile_details() {
        let profile = unsafe { vp::get_profiles().unwrap() }[0];

        unsafe { vp::get_profile_instance_extension_properties(&profile).unwrap() };
        unsafe { vp::get_profile_device_extension_properties(&profile).unwrap() };
        unsafe { vp::get_profile_feature_structure_types(&profile).unwrap() };
        unsafe { vp::get_profile_property_structure_types(&profile).unwrap() };
        unsafe { vp::get_profile_queue_family_structure_types(&profile).unwrap() };
        unsafe { vp::get_profile_formats(&profile).unwrap() };
        unsafe { vp::get_profile_property_structure_types(&profile).unwrap() };
    }

    #[test]
    fn test_create_instance() {
        let (_, instance) = create_instance();

        unsafe { instance.destroy_instance(None) };
    }

    #[test]
    fn test_create_device() {
        let (profile, instance) = create_instance();

        let physical_device = unsafe { instance.enumerate_physical_devices().unwrap() }.into_iter().find(|device| {
            let props = unsafe { instance.get_physical_device_properties(*device) };
            println!("PhysicalDevice: {:?}", unsafe { std::ffi::CStr::from_ptr(props.device_name.as_ptr()) });
            unsafe { vp::get_physical_device_profile_support(&instance, *device, &profile).expect("Error queueing physical device support") }
        }).expect("Failed to find suitable physical device");

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&[1.0]);

        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info));

        let vp_device_info = vp::DeviceCreateInfo::builder()
            .create_info(&device_info)
            .profile(&profile);

        let device = unsafe { vp::create_device(&instance, physical_device, &vp_device_info, None).expect("Failed to create device") };

        unsafe { device.destroy_device(None) };

        unsafe { instance.destroy_instance(None) };

        println!("{:?}", vk::ImageUsageFlags::COLOR_ATTACHMENT);
    }
}
