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

    pub unsafe fn get_profile_fallbacks(p_profile: &ProfileProperties) -> VkResult<Vec<ProfileProperties>> {
        loop { // Basically a copy of how ash does it
            let mut count: u32 = 0;
            sys::vpGetProfileFallbacks(p_profile, &mut count, std::ptr::null_mut()).result()?;
            
            let mut data = Vec::with_capacity(count as usize);
            let err_code = sys::vpGetProfileFallbacks(p_profile, &mut count, data.as_mut_ptr());
            if err_code != vk::Result::INCOMPLETE {
                data.set_len(count as usize);
                break err_code.result_with_success(data);
            }
        }
    }

    // TODO add the layer parameter
    pub unsafe fn get_instance_profile_support(p_profile: &ProfileProperties) -> VkResult<bool> {
        let mut supported: vk::Bool32 = 0;
        sys::vpGetInstanceProfileSupport(std::ptr::null(), p_profile, &mut supported).result()?;
        if supported == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub mod sys {
        use ash::vk;

        #[link(name="vkprofiles", kind="static")]
        extern {
            pub fn vpGetProfiles(pPropertyCount: *mut u32, pProperties: *mut super::ProfileProperties) -> vk::Result;

            pub fn vpGetProfileFallbacks(pProfile: *const super::ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut super::ProfileProperties) -> vk::Result;

            pub fn vpGetInstanceProfileSupport(pLayerName: *const std::os::raw::c_char, pProfile: *const super::ProfileProperties, pSupported: *mut vk::Bool32) -> vk::Result;
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
            //print!("Result: {}", crate::vp::get_instance_profile_support(&profiles[0]).unwrap());
        }
    }
}
