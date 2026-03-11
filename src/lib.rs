//! # Vulkan Profiles Library
//! <https://github.com/KhronosGroup/Vulkan-Profiles>
//!
//! ## Examples
//! ```no_run
//! use ash::vk;
//! use vk_profiles_rs::{profiles, vp};
//!
//! # fn main() -> ash::prelude::VkResult<()> {
//! // Load the function pointers
//! let vk_profiles = vk_profiles_rs::VulkanProfiles::linked();
//!
//! // Select the LunarG minimum Vulkan 1.3 profile and test instance support
//! let profile = profiles::LunargMinimumRequirements1_3::profile_properties();
//! assert!(unsafe { vk_profiles.get_instance_profile_support(None, &profile)? });
//!
//! let instance_info = vk::InstanceCreateInfo::default();
//! let profiles = [profile];
//! let vp_instance_info = vp::InstanceCreateInfo::default()
//!     .create_info(&instance_info)
//!     .enabled_full_profiles(&profiles);
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
//! Currently only static linking is supported. This means that the vulkan loader must also be statically linked in ash.

extern crate link_cplusplus;

#[cfg(feature = "debug")]
#[doc(hidden)]
pub mod enum_debugs;
mod prelude;
pub mod profiles;
pub mod vp;

use ash::prelude::VkResult;
use ash::vk;
use prelude::*;
use std::{
    ffi::{c_char, c_void, CStr},
    ptr,
};
use vp::*;

fn cstr_opt_ptr(cstr_opt: Option<&CStr>) -> *const c_char {
    match cstr_opt {
        Some(cstr) => cstr.as_ptr(),
        None => ptr::null(),
    }
}

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

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profiles>
    pub unsafe fn get_profiles(&self) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| (self.profiles_fn.get_profiles)(count, data))
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-fallbacks>
    pub unsafe fn get_profile_fallbacks(
        &self,
        profile: &ProfileProperties,
    ) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_fallbacks)(profile, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-check-instance-level-support>
    pub unsafe fn get_instance_profile_support(
        &self,
        layer: Option<&CStr>,
        profile: &ProfileProperties,
    ) -> VkResult<bool> {
        let layer = cstr_opt_ptr(layer);

        let mut supported: vk::Bool32 = 0;
        (self.profiles_fn.get_instance_profile_support)(layer, profile, &mut supported).result()?;
        Ok(supported == 1)
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-create-instance-with-profile>
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

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-check-device-level-support>
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

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-create-device-with-profile>
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

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-instance-extensions>
    pub unsafe fn get_profile_instance_extension_properties(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_instance_extension_properties)(
                profile, block_name, count, data,
            )
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-device-extensions>
    pub unsafe fn get_profile_device_extension_properties(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_device_extension_properties)(
                profile, block_name, count, data,
            )
        })
    }

    /// Due to how ash's marker traits work the passed features *must* be wrapped in a [`vk::PhysicalDeviceFeatures2`] struct.
    ///
    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-features>
    pub unsafe fn get_profile_features(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
        features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        let block_name = cstr_opt_ptr(block_name);
        (self.profiles_fn.get_profile_features)(
            profile,
            block_name,
            features as *mut _ as *mut c_void,
        );
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-features>
    pub unsafe fn get_profile_feature_structure_types(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_feature_structure_types)(profile, block_name, count, data)
        })
    }

    /// Due to how ash's marker traits work the passed properties *must* be wrapped in a [`vk::PhysicalDeviceProperties2`] struct.
    ///
    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-device-properties>
    pub unsafe fn get_profile_properties(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
        properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        let block_name = cstr_opt_ptr(block_name);
        (self.profiles_fn.get_profile_properties)(
            profile,
            block_name,
            properties as *mut _ as *mut c_void,
        );
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-device-properties>
    pub unsafe fn get_profile_property_structure_types(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_property_structure_types)(
                profile, block_name, count, data,
            )
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_properties(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
        properties: &mut [vk::QueueFamilyProperties2],
    ) -> VkResult<()> {
        let block_name = cstr_opt_ptr(block_name);
        let mut count = properties.len() as u32;
        (self.profiles_fn.get_profile_queue_family_properties)(
            profile,
            block_name,
            &mut count,
            properties.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, properties.len());
        Ok(())
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_structure_types(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_queue_family_structure_types)(
                profile, block_name, count, data,
            )
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_formats(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::Format>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_formats)(profile, block_name, count, data)
        })
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_format_properties(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
        format: vk::Format,
        p_next: &mut vk::BaseOutStructure,
    ) {
        let block_name = cstr_opt_ptr(block_name);
        (self.profiles_fn.get_profile_format_properties)(
            profile,
            block_name,
            format,
            p_next as *mut _ as *mut c_void,
        );
    }

    /// See <https://vulkan.lunarg.com/doc/view/1.4.335.0/windows/profiles_api_library.html#user-content-query-profile-format-properties>
    pub unsafe fn get_profile_format_structure_types(
        &self,
        profile: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name = cstr_opt_ptr(block_name);
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_format_structure_types)(profile, block_name, count, data)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::profiles;
    use crate::vp;
    use crate::VulkanProfiles;
    use ash::vk;

    fn create_instance(
        entry: &ash::Entry,
        vk_profiles: &VulkanProfiles,
    ) -> ([vp::ProfileProperties; 2], ash::Instance) {
        let profiles = [
            profiles::KhrRoadmap2024::profile_properties(),
            profiles::LunargMinimumRequirements1_3::profile_properties(),
        ];
        for profile in profiles {
            assert!(unsafe {
                vk_profiles
                    .get_instance_profile_support(None, &profile)
                    .unwrap()
            });
        }

        let instance_info = vk::InstanceCreateInfo::default();
        let vp_instance_info = vp::InstanceCreateInfo::default()
            .enabled_full_profiles(&profiles)
            .create_info(&instance_info);

        let instance = unsafe {
            vk_profiles
                .create_instance(entry, &vp_instance_info, None)
                .unwrap()
        };

        (profiles, instance)
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

        let profiles = unsafe { vk_profiles.get_profiles().unwrap() };
        let block_name = None;

        for profile in profiles {
            unsafe {
                vk_profiles
                    .get_profile_instance_extension_properties(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_device_extension_properties(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_feature_structure_types(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_property_structure_types(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_queue_family_structure_types(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_formats(&profile, block_name)
                    .unwrap()
            };
            unsafe {
                vk_profiles
                    .get_profile_property_structure_types(&profile, block_name)
                    .unwrap()
            };
        }
    }

    #[test]
    fn test_create_instance() {
        let entry = ash::Entry::linked();
        let (_, instance) = create_instance(&entry, &VulkanProfiles::linked());

        unsafe { instance.destroy_instance(None) };
    }

    #[test]
    fn test_create_device() {
        let vk_profiles = VulkanProfiles::linked();
        let entry = ash::Entry::linked();

        let (profiles, instance) = create_instance(&entry, &vk_profiles);

        let physical_device = unsafe { instance.enumerate_physical_devices().unwrap() }
            .into_iter()
            .find(|device| {
                let props = unsafe { instance.get_physical_device_properties(*device) };
                println!("PhysicalDevice: {:?}", unsafe {
                    std::ffi::CStr::from_ptr(props.device_name.as_ptr())
                });
                let mut unsupported = false;
                for profile in profiles {
                    let supports_this = unsafe {
                        vk_profiles
                            .get_physical_device_profile_support(&instance, *device, &profile)
                            .expect("Error queueing physical device support")
                    };
                    if !supports_this {
                        unsupported = true;
                        break;
                    }
                }
                !unsupported
            })
            .expect("Failed to find suitable physical device");

        let queue_priorities: [f32; 1] = [1.0];
        let queue_info = vk::DeviceQueueCreateInfo {
            queue_family_index: 0,
            p_queue_priorities: queue_priorities.as_ptr(),
            ..Default::default()
        };

        let device_info = vk::DeviceCreateInfo {
            p_queue_create_infos: std::ptr::addr_of!(queue_info),
            ..Default::default()
        };

        let vp_device_info = vp::DeviceCreateInfo::default()
            .enabled_full_profiles(&profiles)
            .create_info(&device_info)
            .flags(vp::DeviceCreateFlagBits::DISABLE_ROBUST_ACCESS);
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
