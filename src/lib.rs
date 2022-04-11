pub mod vp {
    use ash::prelude::VkResult;
    use ash::vk;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ProfileProperties {
        pub profile_name: [std::os::raw::c_char; 256],
        pub spec_version: u32,
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InstanceCreateFlagBits(pub(crate) vk::Flags);
    ash::vk_bitflags_wrapped!(InstanceCreateFlagBits, vk::Flags);

    impl InstanceCreateFlagBits {
        pub const MERGE_EXTENSIONS: Self = Self(0x00000001);
        pub const OVERRIDE_EXTENSIONS: Self = Self(0x00000002);
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct InstanceCreateInfo {
        pub p_create_info: *const vk::InstanceCreateInfo,
        pub p_profile: *const ProfileProperties,
        pub flags: InstanceCreateFlagBits,
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceCreateFlagBits(pub(crate) vk::Flags);
    ash::vk_bitflags_wrapped!(DeviceCreateFlagBits, vk::Flags);

    impl DeviceCreateFlagBits {
        pub const MERGE_EXTENSIONS: Self = Self(0x00000001);
        pub const OVERRIDE_EXTENSIONS: Self = Self(0x00000002);
        pub const OVERRIDE_FEATURES: Self = Self(0x00000008);
        pub const OVERRIDE_ALL_FEATURES: Self = Self(0x00000010);
        pub const DISABLE_ROBUST_BUFFER_ACCESS: Self = Self(0x00000020);
        pub const DISABLE_ROBUST_IMAGE_ACCESS: Self = Self(0x00000040);
        pub const DISABLE_ROBUST_ACCESS: Self = Self(0x00000020 | 0x00000040);
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceCreateInfo {
        pub p_create_info: *const vk::DeviceCreateInfo,
        pub p_profile: *const ProfileProperties,
        pub flags: DeviceCreateFlagBits,
    }

    pub unsafe fn get_profiles() -> VkResult<Vec<ProfileProperties>> {
        loop { // Basically a copy of how ash does it
            let mut count: u32 = 0;
            sys::vpGetProfiles(&mut count, std::ptr::null_mut()).result()?;
            
            let mut data = Vec::with_capacity(count as usize);
            let err_code = sys::vpGetProfiles(&mut count, data.as_mut_ptr());
            if err_code != vk::Result::INCOMPLETE {
                data.set_len(count as usize);
                break err_code.result_with_success(data);
            }
        }
    }

    pub unsafe fn get_profile_fallbacks(profile: &ProfileProperties) -> VkResult<Vec<ProfileProperties>> {
        loop { // Basically a copy of how ash does it
            let mut count: u32 = 0;
            sys::vpGetProfileFallbacks(profile, &mut count, std::ptr::null_mut()).result()?;
            
            let mut data = Vec::with_capacity(count as usize);
            let err_code = sys::vpGetProfileFallbacks(profile, &mut count, data.as_mut_ptr());
            if err_code != vk::Result::INCOMPLETE {
                data.set_len(count as usize);
                break err_code.result_with_success(data);
            }
        }
    }

    // TODO add the layer parameter
    pub unsafe fn get_instance_profile_support(profile: &ProfileProperties) -> VkResult<bool> {
        let mut supported: vk::Bool32 = 0;
        sys::vpGetInstanceProfileSupport(std::ptr::null(), profile, &mut supported).result()?;
        if supported == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub unsafe fn create_instance(entry: &ash::Entry, create_info: &InstanceCreateInfo, allocator: Option<vk::AllocationCallbacks>) -> VkResult<ash::Instance> {
        let allocator = match allocator {
            Some(allocator) => &allocator,
            _ => std::ptr::null(),
        };

        let mut instance = std::mem::zeroed();
        sys::vpCreateInstance(create_info, allocator, &mut instance).result()?;
        Ok(ash::Instance::load(entry.static_fn(), instance))
    }

    pub unsafe fn get_physical_device_profile_support(instance: &ash::Instance, physical_device: vk::PhysicalDevice, profile: &ProfileProperties) -> VkResult<bool> {
        let mut supported: vk::Bool32 = 0;
        sys::vpGetPhysicalDeviceProfileSupport(instance.handle(), physical_device, profile, &mut supported).result()?;
        if supported == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub unsafe fn create_device(instance: &ash::Instance, physical_device: vk::PhysicalDevice, create_info: &DeviceCreateInfo, allocator: Option<vk::AllocationCallbacks>) -> VkResult<ash::Device> {
        let allocator = match allocator {
            Some(allocator) => &allocator,
            _ => std::ptr::null(),
        };

        let mut device = std::mem::zeroed();
        sys::vpCreateDevice(physical_device, create_info, allocator, &mut device).result()?;
        Ok(ash::Device::load(instance.fp_v1_0(), device))
    }

    pub mod sys {
        use ash::vk;

        #[link(name="vkprofiles", kind="static")]
        extern {
            pub fn vpGetProfiles(pPropertyCount: *mut u32, pProperties: *mut super::ProfileProperties) -> vk::Result;

            pub fn vpGetProfileFallbacks(pProfile: *const super::ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut super::ProfileProperties) -> vk::Result;

            pub fn vpGetInstanceProfileSupport(pLayerName: *const std::os::raw::c_char, pProfile: *const super::ProfileProperties, pSupported: *mut vk::Bool32) -> vk::Result;

            pub fn vpCreateInstance(pCreateInfo: *const super::InstanceCreateInfo, pAllocator: *const vk::AllocationCallbacks, p_instance: *mut vk::Instance) -> vk::Result;

            pub fn vpGetPhysicalDeviceProfileSupport(instance: ash::vk::Instance, physicalDevice: ash::vk::PhysicalDevice, pProfile: *const super::ProfileProperties, supported: *mut vk::Bool32) -> vk::Result;

            pub fn vpCreateDevice(physicalDevice: ash::vk::PhysicalDevice, pCreateInfo: *const super::DeviceCreateInfo, pAllocator: *const vk::AllocationCallbacks, pDevice: *mut vk::Device) -> vk::Result;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            let profiles = crate::vp::get_profiles().unwrap();
            assert!(profiles.len() > 0);
            let _test = crate::vp::get_profile_fallbacks(&profiles[0]).unwrap();
            print!("Result: {}", crate::vp::get_instance_profile_support(&profiles[0]).unwrap());
        }
    }
}
