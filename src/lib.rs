//! # Vulkan Profiles Library
//! <https://github.com/KhronosGroup/Vulkan-Profiles>
//!
//! ## Examples
//! ```no_run
//! use ash::vk;
//! use vk_profiles_rs::vp;
//!
//! # fn main() -> ash::prelude::VkResult<()> {
//! // Load the function pointers
//! let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();
//!
//! // Select the lunarg desktop portability 2021 profile and test instance support
//! let profile = vp::LunargDesktopPortability2021::profile_properties();
//! assert!(unsafe { vk_profiles.get_instance_profile_support(None, &profile)? });
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
//! let instance = unsafe { vk_profiles.create_instance(&entry, &vp_instance_info, None)? };
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

#[cfg(feature = "debug")]
#[doc(hidden)]
pub mod enum_debugs;
mod prelude;
pub mod vp;

use ash::prelude::VkResult;
use ash::vk;
use prelude::*;
use std::ffi::c_void;
use vp::*;

/// A wrapper struct that provides access to the vulkan profiles functions.
#[derive(Clone)]
pub struct VulkanProfiles {
    profiles_fn: vp::ProfilesFn,
}

impl VulkanProfiles {
    /// Loads the function pointers when the vulkan profiles library is statically 
    /// linked (which is currently the only option).
    pub fn linked() -> Self {
        VulkanProfiles {
            profiles_fn: vp::ProfilesFn::load_static(),
        }
    }

    /// Returns the raw function pointer table
    pub fn profiles_fn(&self) -> &vp::ProfilesFn {
        &self.profiles_fn
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profiles>
    pub unsafe fn get_profiles(&self) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| (self.profiles_fn.get_profiles)(count, data))
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-fallbacks>
    pub unsafe fn get_profile_fallbacks(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_fallbacks)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-check-instance-level-support>
    pub unsafe fn get_instance_profile_support(
        &self,
        layer: Option<&::std::ffi::CStr>,
        profile: &ProfileProperties,
    ) -> VkResult<bool> {
        let layer = match layer {
            Some(layer) => layer.as_ptr(),
            _ => std::ptr::null(),
        };

        let mut supported: vk::Bool32 = 0;
        (self.profiles_fn.get_instance_profile_support)(layer, profile, &mut supported).result()?;
        if supported == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-create-instance-with-profile>
    pub unsafe fn create_instance(
        &self,
        entry: &ash::Entry,
        create_info: &InstanceCreateInfo,
        allocator: Option<vk::AllocationCallbacks>,
    ) -> VkResult<ash::Instance> {
        let allocator = match allocator {
            Some(allocator) => &allocator,
            _ => std::ptr::null(),
        };

        let mut instance = std::mem::zeroed();
        (self.profiles_fn.create_instance)(create_info, allocator, &mut instance).result()?;
        Ok(ash::Instance::load(entry.static_fn(), instance))
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-check-device-level-support>
    pub unsafe fn get_physical_device_profile_support(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        profile: &ProfileProperties,
    ) -> VkResult<bool> {
        let mut supported: vk::Bool32 = 0;
        (self.profiles_fn.get_physical_device_profile_support)(
            instance.handle(),
            physical_device,
            profile,
            &mut supported,
        )
        .result()?;
        if supported == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-create-device-with-profile>
    pub unsafe fn create_device(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        create_info: &DeviceCreateInfo,
        allocator: Option<vk::AllocationCallbacks>,
    ) -> VkResult<ash::Device> {
        let allocator = match allocator {
            Some(allocator) => &allocator,
            _ => std::ptr::null(),
        };

        let mut device = std::mem::zeroed();
        (self.profiles_fn.create_device)(physical_device, create_info, allocator, &mut device)
            .result()?;
        Ok(ash::Device::load(instance.fp_v1_0(), device))
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-instance-extensions>
    pub unsafe fn get_profile_instance_extension_properties(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_instance_extension_properties)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-device-extensions>
    pub unsafe fn get_profile_device_extension_properties(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_device_extension_properties)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-features>
    pub unsafe fn get_profile_features(
        &self,
        profile: &ProfileProperties,
        p_next: &mut vk::BaseOutStructure,
    ) {
        (self.profiles_fn.get_profile_features)(profile, p_next as *mut _ as *mut c_void);
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-features>
    pub unsafe fn get_profile_feature_structure_types(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_feature_structure_types)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-device-properties>
    pub unsafe fn get_profile_properties(
        &self,
        profile: &ProfileProperties,
        p_next: &mut vk::BaseOutStructure,
    ) {
        (self.profiles_fn.get_profile_properties)(profile, p_next as *mut _ as *mut c_void);
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-device-properties>
    pub unsafe fn get_profile_property_structure_types(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_property_structure_types)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_properties(
        &self,
        profile: &ProfileProperties,
        properties: &mut [vk::QueueFamilyProperties2],
    ) -> VkResult<()> {
        let mut count = properties.len() as u32;
        (self.profiles_fn.get_profile_queue_family_properties)(
            profile,
            &mut count,
            properties.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, properties.len());
        Ok(())
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_structure_types(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_queue_family_structure_types)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_formats(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::Format>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_formats)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_format_properties(
        &self,
        profile: &ProfileProperties,
        format: vk::Format,
        p_next: &mut vk::BaseOutStructure,
    ) {
        (self.profiles_fn.get_profile_format_properties)(
            profile,
            format,
            p_next as *mut _ as *mut c_void,
        );
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.3.204.1/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_format_structure_types(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_format_structure_types)(profile, count, data)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::vp;
    use crate::VulkanProfiles;
    use ash::vk;

    fn create_instance(profiles: &VulkanProfiles) -> (vp::ProfileProperties, ash::Instance) {
        let profile = vp::LunargDesktopPortability2021::profile_properties();
        assert!(unsafe {
            profiles
                .get_instance_profile_support(None, &profile)
                .unwrap()
        });

        let entry = ash::Entry::linked();

        let instance_info = vk::InstanceCreateInfo::builder();
        let vp_instance_info = vp::InstanceCreateInfo::builder()
            .create_info(&instance_info)
            .profile(&profile);

        let instance = unsafe {
            profiles
                .create_instance(&entry, &vp_instance_info, None)
                .unwrap()
        };

        (profile, instance)
    }

    #[test]
    fn test_enumerate_profiles() {
        let vk_profiles = VulkanProfiles::linked();

        let profiles = unsafe { vk_profiles.get_profiles().unwrap() };

        assert!(profiles.len() > 0);
        for profile in &profiles {
            println!(
                "Profile {:?}: {:?}",
                unsafe {
                    vk_profiles
                        .get_instance_profile_support(None, profile)
                        .unwrap()
                },
                profile
            );
        }

        unsafe { vk_profiles.get_profile_fallbacks(&profiles[0]).unwrap() };
    }

    #[test]
    fn test_enumerate_profile_details() {
        let vk_profiles = VulkanProfiles::linked();

        let profile = unsafe { vk_profiles.get_profiles().unwrap() }[0];

        unsafe {
            vk_profiles
                .get_profile_instance_extension_properties(&profile)
                .unwrap()
        };
        unsafe {
            vk_profiles
                .get_profile_device_extension_properties(&profile)
                .unwrap()
        };
        unsafe {
            vk_profiles
                .get_profile_feature_structure_types(&profile)
                .unwrap()
        };
        unsafe {
            vk_profiles
                .get_profile_property_structure_types(&profile)
                .unwrap()
        };
        unsafe {
            vk_profiles
                .get_profile_queue_family_structure_types(&profile)
                .unwrap()
        };
        unsafe { vk_profiles.get_profile_formats(&profile).unwrap() };
        unsafe {
            vk_profiles
                .get_profile_property_structure_types(&profile)
                .unwrap()
        };
    }

    #[test]
    fn test_create_instance() {
        let (_, instance) = create_instance(&VulkanProfiles::linked());

        unsafe { instance.destroy_instance(None) };
    }

    #[test]
    fn test_create_device() {
        let vk_profiles = VulkanProfiles::linked();

        let (profile, instance) = create_instance(&vk_profiles);

        let physical_device = unsafe { instance.enumerate_physical_devices().unwrap() }
            .into_iter()
            .find(|device| {
                let props = unsafe { instance.get_physical_device_properties(*device) };
                println!("PhysicalDevice: {:?}", unsafe {
                    std::ffi::CStr::from_ptr(props.device_name.as_ptr())
                });
                unsafe {
                    vk_profiles
                        .get_physical_device_profile_support(&instance, *device, &profile)
                        .expect("Error queueing physical device support")
                }
            })
            .expect("Failed to find suitable physical device");

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&[1.0]);

        let device_info =
            vk::DeviceCreateInfo::builder().queue_create_infos(std::slice::from_ref(&queue_info));

        let vp_device_info = vp::DeviceCreateInfo::builder()
            .create_info(&device_info)
            .profile(&profile);

        let device = unsafe {
            vk_profiles
                .create_device(&instance, physical_device, &vp_device_info, None)
                .expect("Failed to create device")
        };

        unsafe { device.destroy_device(None) };

        unsafe { instance.destroy_instance(None) };

        println!("{:?}", vk::ImageUsageFlags::COLOR_ATTACHMENT);
    }
}
