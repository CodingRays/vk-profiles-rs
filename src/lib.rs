extern crate link_cplusplus;

pub mod vp {
    use ash::prelude::VkResult;
    use std::ffi::c_void;
    use ash::vk;

    pub struct AndroidBaseline2021;
    impl AndroidBaseline2021 {
        pub const NAME: &'static ::std::ffi::CStr =
            unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VP_ANDROID_baseline_2021\0") };

        pub const SPEC_VERSION: u32 = 1;
        pub const MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 0, 68);

        pub fn profile_properties() -> ProfileProperties {
            ProfileProperties {
                profile_name: c_char_array_from_cstr(Self::NAME).unwrap(),
                spec_version: Self::SPEC_VERSION,
            }
        }
    }

    pub struct KhrRoadmap2022 ;
    impl KhrRoadmap2022 {
        pub const NAME: &'static ::std::ffi::CStr =
            unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VP_KHR_roadmap_2022\0") };

        pub const SPEC_VERSION: u32 = 1;
        pub const MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 3, 204);

        pub fn profile_properties() -> ProfileProperties {
            ProfileProperties {
                profile_name: c_char_array_from_cstr(Self::NAME).unwrap(),
                spec_version: Self::SPEC_VERSION,
            }
        }
    }

    pub struct LunargDesktopPortability2021;
    impl LunargDesktopPortability2021 {
        pub const NAME: &'static ::std::ffi::CStr =
            unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VP_LUNARG_desktop_portability_2021\0") };

        pub const SPEC_VERSION: u32 = 1;
        pub const MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 1, 142);

        pub fn profile_properties() -> ProfileProperties {
            ProfileProperties {
                profile_name: c_char_array_from_cstr(Self::NAME).unwrap(),
                spec_version: Self::SPEC_VERSION,
            }
        }
    }

    pub struct LunargDesktopPortability2021Subset;
    impl LunargDesktopPortability2021Subset {
        pub const NAME: &'static ::std::ffi::CStr =
            unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VP_LUNARG_desktop_portability_2021_subset\0") };

        pub const SPEC_VERSION: u32 = 1;
        pub const MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 1, 154);

        pub fn profile_properties() -> ProfileProperties {
            ProfileProperties {
                profile_name: c_char_array_from_cstr(Self::NAME).unwrap(),
                spec_version: Self::SPEC_VERSION,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ProfileProperties {
        pub profile_name: [std::os::raw::c_char; 256],
        pub spec_version: u32,
    }
    impl std::fmt::Debug for ProfileProperties {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
             fmt.debug_struct("ProfileProperties")
                .field("profile_name", &unsafe {
                    ::std::ffi::CStr::from_ptr(self.profile_name.as_ptr())
                })
                .field("spec_version", &self.spec_version)
                .finish()
        }
    }
    impl ::std::default::Default for ProfileProperties {
        fn default() -> Self {
            Self {
                profile_name: unsafe { ::std::mem::zeroed() },
                spec_version: u32::default(),
            }
        }
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
    impl ::std::default::Default for InstanceCreateInfo {
        fn default() -> Self {
            Self {
                p_create_info: std::ptr::null(),
                p_profile: std::ptr::null(),
                flags: InstanceCreateFlagBits::default(),
            }
        }
    }
    impl InstanceCreateInfo {
        pub fn builder<'a>() -> InstanceCreateInfoBuilder<'a> {
            InstanceCreateInfoBuilder {
                inner: Self::default(),
                marker: ::std::marker::PhantomData,
            }
        }
    }

    #[repr(transparent)]
    pub struct InstanceCreateInfoBuilder<'a> {
        inner: InstanceCreateInfo,
        marker: ::std::marker::PhantomData<&'a ()>,
    }
    impl<'a> ::std::ops::Deref for InstanceCreateInfoBuilder<'a> {
        type Target = InstanceCreateInfo;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
    impl<'a> ::std::ops::DerefMut for InstanceCreateInfoBuilder<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
    impl<'a> InstanceCreateInfoBuilder<'a> {
        pub fn create_info(mut self, create_info: &'a vk::InstanceCreateInfo) -> Self {
            self.inner.p_create_info = create_info;
            self
        }

        pub fn profile(mut self, profile: &'a ProfileProperties) -> Self {
            self.inner.p_profile = profile;
            self
        }

        pub fn flags(mut self, flags: InstanceCreateFlagBits) -> Self {
            self.inner.flags = flags;
            self
        }

        /// Calling build will **discard** all the lifetime information. Only call this if
        /// necessary! Builders implement `Deref` targeting their corresponding Vulkan-Profiles struct,
        /// so references to builders can be passed directly to Vulkan-Profiles functions.
        pub fn build(self) -> InstanceCreateInfo {
            self.inner
        }
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
    impl ::std::default::Default for DeviceCreateInfo {
        fn default() -> Self {
            Self {
                p_create_info: std::ptr::null(),
                p_profile: std::ptr::null(),
                flags: DeviceCreateFlagBits::default(),
            }
        }
    }
    impl DeviceCreateInfo {
        pub fn builder<'a>() -> DeviceCreateInfoBuilder<'a> {
            DeviceCreateInfoBuilder {
                inner: Self::default(),
                marker: ::std::marker::PhantomData,
            }
        }
    }

    #[repr(transparent)]
    pub struct DeviceCreateInfoBuilder<'a> {
        inner: DeviceCreateInfo,
        marker: ::std::marker::PhantomData<&'a ()>,
    }
    impl<'a> ::std::ops::Deref for DeviceCreateInfoBuilder<'a> {
        type Target = DeviceCreateInfo;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
    impl<'a> ::std::ops::DerefMut for DeviceCreateInfoBuilder<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
    impl<'a> DeviceCreateInfoBuilder<'a> {
        pub fn create_info(mut self, create_info: &'a vk::DeviceCreateInfo) -> Self {
            self.inner.p_create_info = create_info;
            self
        }

        pub fn profile(mut self, profile: &'a ProfileProperties) -> Self {
            self.inner.p_profile = profile;
            self
        }

        pub fn flags(mut self, flags: DeviceCreateFlagBits) -> Self {
            self.inner.flags = flags;
            self
        }

        /// Calling build will **discard** all the lifetime information. Only call this if
        /// necessary! Builders implement `Deref` targeting their corresponding Vulkan-Profiles struct,
        /// so references to builders can be passed directly to Vulkan-Profiles functions.
        pub fn build(self) -> DeviceCreateInfo {
            self.inner
        }
    }

    pub unsafe fn get_profiles() -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfiles(count, data)
        })
    }

    pub unsafe fn get_profile_fallbacks(profile: &ProfileProperties) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileFallbacks(profile, count, data)
        })
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

    pub unsafe fn get_profile_instance_extension_properties(profile: &ProfileProperties) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileInstanceExtensionProperties(profile, count, data)
        })
    }

    pub unsafe fn get_profile_device_extension_properties(profile: &ProfileProperties) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileDeviceExtensionProperties(profile, count, data)
        })
    }

    pub unsafe fn get_profile_features(profile: &ProfileProperties, p_next: &mut vk::BaseOutStructure) {
        sys::vpGetProfileFeatures(profile, p_next as *mut _ as *mut c_void);
    }

    pub unsafe fn get_profile_feature_structure_types(profile: &ProfileProperties) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileFeatureStructureTypes(profile, count, data)
        })
    }

    pub unsafe fn get_profile_properties(profile: &ProfileProperties, p_next: &mut vk::BaseOutStructure) {
        sys::vpGetProfileProperties(profile, p_next as *mut _ as *mut c_void);
    }

    pub unsafe fn get_profile_property_structure_types(profile: &ProfileProperties) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfilePropertyStructureTypes(profile, count, data)
        })
    }

    pub unsafe fn get_profile_queue_family_properties(profile: &ProfileProperties, properties: &mut [vk::QueueFamilyProperties2]) -> VkResult<()> {
        let mut count = properties.len() as u32;
        sys::vpGetProfileQueueFamilyProperties(profile, &mut count, properties.as_mut_ptr()).result()?;
        assert_eq!(count as usize, properties.len());
        Ok(())
    }

    pub unsafe fn get_profile_queue_family_structure_types(profile: &ProfileProperties) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileQueueFamilyStructureTypes(profile, count, data)
        })
    }

    pub unsafe fn get_profile_formats(profile: &ProfileProperties) -> VkResult<Vec<vk::Format>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileFormats(profile, count, data)
        })
    }

    pub unsafe fn get_profile_format_properties(profile: &ProfileProperties, format: vk::Format, p_next: &mut vk::BaseOutStructure) {
        sys::vpGetProfileFormatProperties(profile, format, p_next as *mut _ as *mut c_void);
    }

    pub unsafe fn get_profile_format_structure_types(profile: &ProfileProperties) -> VkResult<Vec<vk::StructureType>> {
        read_into_uninitialized_vector(|count, data| {
            sys::vpGetProfileFormatStructureTypes(profile, count, data)
        })
    }

    pub mod sys {
        use super::*;

        #[link(name="vkprofiles", kind="static")]
        extern {
            pub fn vpGetProfiles(pPropertyCount: *mut u32, pProperties: *mut ProfileProperties) -> vk::Result;

            pub fn vpGetProfileFallbacks(pProfile: *const ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut ProfileProperties) -> vk::Result;

            pub fn vpGetInstanceProfileSupport(pLayerName: *const std::os::raw::c_char, pProfile: *const ProfileProperties, pSupported: *mut vk::Bool32) -> vk::Result;

            pub fn vpCreateInstance(pCreateInfo: *const InstanceCreateInfo, pAllocator: *const vk::AllocationCallbacks, p_instance: *mut vk::Instance) -> vk::Result;

            pub fn vpGetPhysicalDeviceProfileSupport(instance: ash::vk::Instance, physicalDevice: ash::vk::PhysicalDevice, pProfile: *const ProfileProperties, supported: *mut vk::Bool32) -> vk::Result;

            pub fn vpCreateDevice(physicalDevice: ash::vk::PhysicalDevice, pCreateInfo: *const DeviceCreateInfo, pAllocator: *const vk::AllocationCallbacks, pDevice: *mut vk::Device) -> vk::Result;

            pub fn vpGetProfileInstanceExtensionProperties(pProfile: *const ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut vk::ExtensionProperties) -> vk::Result;

            pub fn vpGetProfileDeviceExtensionProperties(pProfile: *const ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut vk::ExtensionProperties) -> vk::Result;

            pub fn vpGetProfileFeatures(pProfile: *const ProfileProperties, pNext: *mut c_void);

            pub fn vpGetProfileFeatureStructureTypes(pProfile: *const ProfileProperties, pStructureTypeCount: *mut u32, pStructureTypes: *mut vk::StructureType) -> vk::Result;

            pub fn vpGetProfileProperties(pProfile: *const ProfileProperties, pNext: *mut c_void);

            pub fn vpGetProfilePropertyStructureTypes(pProfile: *const ProfileProperties, pStructureTypeCount: *mut u32, pStructureTypes: *mut vk::StructureType) -> vk::Result;

            pub fn vpGetProfileQueueFamilyProperties(pProfile: *const ProfileProperties, pPropertyCount: *mut u32, pProperties: *mut vk::QueueFamilyProperties2) -> vk::Result;

            pub fn vpGetProfileQueueFamilyStructureTypes(pProfile: *const ProfileProperties, pStructureTypeCount: *mut u32, pStructureTypes: *mut vk::StructureType) -> vk::Result;

            pub fn vpGetProfileFormats(pProfile: *const ProfileProperties, pFormatCount: *mut u32, pFormats: *mut vk::Format) -> vk::Result;

            pub fn vpGetProfileFormatProperties(pProfile: *const ProfileProperties, format: vk::Format, pNext: *mut c_void);

            pub fn vpGetProfileFormatStructureTypes(pProfile: *const ProfileProperties, pStructureTypeCount: *mut u32, pStructureTypes: *mut vk::StructureType) -> vk::Result;
        }
    }

    /// This is a direct copy from ash::prelude (because it is not public).
    ///
    /// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore,
    /// ensuring all available data has been read into the vector.
    ///
    /// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available
    /// items may change between calls; [`vk::Result::INCOMPLETE`] is returned when the count
    /// increased (and the vector is not large enough after querying the initial size),
    /// requiring Ash to try again.
    ///
    /// [`vkEnumerateInstanceExtensionProperties`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
    pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
        f: impl Fn(&mut N, *mut T) -> vk::Result,
    ) -> VkResult<Vec<T>>
    where
        <N as TryInto<usize>>::Error: std::fmt::Debug,
    {
        loop {
            let mut count = N::default();
            f(&mut count, std::ptr::null_mut()).result()?;
            let mut data =
                Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

            let err_code = f(&mut count, data.as_mut_ptr());
            if err_code != vk::Result::INCOMPLETE {
                data.set_len(count.try_into().expect("`N` failed to convert to `usize`"));
                break err_code.result_with_success(data);
            }
        }
    }

    pub(crate) fn c_char_array_from_cstr<const N: usize>(data: &::std::ffi::CStr) -> Option<[::std::os::raw::c_char; N]> {
        let mut result: [::std::os::raw::c_char; N] = unsafe { ::std::mem::zeroed() }; // Default not implemented for arbitrary length

        // Yes this is stupid but rust FFI is absolutely useless
        let data = data.to_bytes_with_nul();
        if data.len() > N {
            return None;
        }

        for (i, c) in data.iter().enumerate() {
            result[i] = *c as ::std::os::raw::c_char;
        }

        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use crate::vp;
    use ash::vk;

    fn create_instance() -> (vp::ProfileProperties, ash::Instance) {
        // let profile = unsafe { vp::get_profiles().unwrap() }.into_iter().find(|profile| {
        //     unsafe { vp::get_instance_profile_support(profile).unwrap() }
        // }).unwrap();
        // println!("Found supported profile: {:?}", profile);

        let profile = vp::LunargDesktopPortability2021::profile_properties();
        assert!(unsafe { vp::get_instance_profile_support(&profile).unwrap() });

        let entry = unsafe { ash::Entry::load().unwrap() };

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
            println!("Profile {:?}: {:?}", unsafe { vp::get_instance_profile_support(profile).unwrap()}, profile);
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
    }
}
