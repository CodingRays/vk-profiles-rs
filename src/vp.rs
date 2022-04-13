//! The vulkan profiles structures and function definitions.
//! 
//! See the vulkan profiles documentation for more details <https://vulkan.lunarg.com/doc/sdk/1.3.204.1/windows/profiles_api_library.html>

use crate::prelude::*;

use ash::vk;
use std::ffi::c_void;

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

pub struct KhrRoadmap2022;
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
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VP_LUNARG_desktop_portability_2021\0")
    };

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
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VP_LUNARG_desktop_portability_2021_subset\0",
        )
    };

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
#[cfg(feature = "debug")]
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
#[cfg_attr(feature = "debug", derive(Debug))]
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
#[cfg_attr(feature = "debug", derive(Debug))]
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

/// Holds all the function pointers of the vulkan profiles library
#[derive(Clone)]
pub struct ProfilesFn {
    pub get_profiles: PFN_vpGetProfiles,
    pub get_profile_fallbacks: PFN_vpGetProfileFallbacks,
    pub get_instance_profile_support: PFN_vpGetInstanceProfileSupport,
    pub create_instance: PFN_vpCreateInstance,
    pub get_physical_device_profile_support: PFN_vpGetPhysicalDeviceProfileSupport,
    pub create_device: PFN_vpCreateDevice,
    pub get_profile_instance_extension_properties: PFN_vpGetProfileInstanceExtensionProperties,
    pub get_profile_device_extension_properties: PFN_vpGetProfileDeviceExtensionProperties,
    pub get_profile_features: PFN_vpGetProfileFeatures,
    pub get_profile_feature_structure_types: PFN_vpGetProfileFeatureStructureTypes,
    pub get_profile_properties: PFN_vpGetProfileProperties,
    pub get_profile_property_structure_types: PFN_vpGetProfilePropertyStructureTypes,
    pub get_profile_queue_family_properties: PFN_vpGetProfileQueueFamilyProperties,
    pub get_profile_queue_family_structure_types: PFN_vpGetProfileQueueFamilyStructureTypes,
    pub get_profile_formats: PFN_vpGetProfileFormats,
    pub get_profile_format_properties: PFN_vpGetProfileFormatProperties,
    pub get_profile_format_structure_types: PFN_vpGetProfileFormatStructureTypes,
}
unsafe impl Send for ProfilesFn {}
unsafe impl Sync for ProfilesFn {}
impl ProfilesFn {
    /// Initializes the table from a statically linked library
    pub fn load_static() -> Self {
        Self {
            get_profiles: sys::vpGetProfiles,
            get_profile_fallbacks: sys::vpGetProfileFallbacks,
            get_instance_profile_support: sys::vpGetInstanceProfileSupport,
            create_instance: sys::vpCreateInstance,
            get_physical_device_profile_support: sys::vpGetPhysicalDeviceProfileSupport,
            create_device: sys::vpCreateDevice,
            get_profile_instance_extension_properties: sys::vpGetProfileInstanceExtensionProperties,
            get_profile_device_extension_properties: sys::vpGetProfileDeviceExtensionProperties,
            get_profile_features: sys::vpGetProfileFeatures,
            get_profile_feature_structure_types: sys::vpGetProfileFeatureStructureTypes,
            get_profile_properties: sys::vpGetProfileProperties,
            get_profile_property_structure_types: sys::vpGetProfilePropertyStructureTypes,
            get_profile_queue_family_properties: sys::vpGetProfileQueueFamilyProperties,
            get_profile_queue_family_structure_types: sys::vpGetProfileQueueFamilyStructureTypes,
            get_profile_formats: sys::vpGetProfileFormats,
            get_profile_format_properties: sys::vpGetProfileFormatProperties,
            get_profile_format_structure_types: sys::vpGetProfileFormatStructureTypes,
        }
    }
}

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfiles = unsafe extern "C" fn(
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFallbacks = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetInstanceProfileSupport = unsafe extern "C" fn(
    pLayerName: *const std::os::raw::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const InstanceCreateInfo,
    pAllocator: *const vk::AllocationCallbacks,
    p_instance: *mut vk::Instance,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetPhysicalDeviceProfileSupport = unsafe extern "C" fn(
    instance: ash::vk::Instance,
    physicalDevice: ash::vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    supported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateDevice = unsafe extern "C" fn(
    physicalDevice: ash::vk::PhysicalDevice,
    pCreateInfo: *const DeviceCreateInfo,
    pAllocator: *const vk::AllocationCallbacks,
    pDevice: *mut vk::Device,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileInstanceExtensionProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileDeviceExtensionProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatures = unsafe extern "C" fn(
    pProfile: *const ProfileProperties, 
    pNext: *mut c_void
);

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatureStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties, 
    pNext: *mut c_void
);

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfilePropertyStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileQueueFamilyProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::QueueFamilyProperties2,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileQueueFamilyStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormats = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pFormatCount: *mut u32,
    pFormats: *mut vk::Format,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    format: vk::Format,
    pNext: *mut c_void,
);

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

mod sys {
    //! External function definitions when statically linked.
    //! 
    //! If raw access to these functions is needed use the [crate::VulkanProfiles::profiles_fn]
    //! function to get the function pointer table.

    use super::*;

    #[link(name = "vkprofiles", kind = "static")]
    extern "C" {
        pub fn vpGetProfiles(
            pPropertyCount: *mut u32,
            pProperties: *mut ProfileProperties,
        ) -> vk::Result;

        pub fn vpGetProfileFallbacks(
            pProfile: *const ProfileProperties,
            pPropertyCount: *mut u32,
            pProperties: *mut ProfileProperties,
        ) -> vk::Result;

        pub fn vpGetInstanceProfileSupport(
            pLayerName: *const std::os::raw::c_char,
            pProfile: *const ProfileProperties,
            pSupported: *mut vk::Bool32,
        ) -> vk::Result;

        pub fn vpCreateInstance(
            pCreateInfo: *const InstanceCreateInfo,
            pAllocator: *const vk::AllocationCallbacks,
            p_instance: *mut vk::Instance,
        ) -> vk::Result;

        pub fn vpGetPhysicalDeviceProfileSupport(
            instance: ash::vk::Instance,
            physicalDevice: ash::vk::PhysicalDevice,
            pProfile: *const ProfileProperties,
            supported: *mut vk::Bool32,
        ) -> vk::Result;

        pub fn vpCreateDevice(
            physicalDevice: ash::vk::PhysicalDevice,
            pCreateInfo: *const DeviceCreateInfo,
            pAllocator: *const vk::AllocationCallbacks,
            pDevice: *mut vk::Device,
        ) -> vk::Result;

        pub fn vpGetProfileInstanceExtensionProperties(
            pProfile: *const ProfileProperties,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::ExtensionProperties,
        ) -> vk::Result;

        pub fn vpGetProfileDeviceExtensionProperties(
            pProfile: *const ProfileProperties,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::ExtensionProperties,
        ) -> vk::Result;

        pub fn vpGetProfileFeatures(pProfile: *const ProfileProperties, pNext: *mut c_void);

        pub fn vpGetProfileFeatureStructureTypes(
            pProfile: *const ProfileProperties,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileProperties(pProfile: *const ProfileProperties, pNext: *mut c_void);

        pub fn vpGetProfilePropertyStructureTypes(
            pProfile: *const ProfileProperties,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileQueueFamilyProperties(
            pProfile: *const ProfileProperties,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::QueueFamilyProperties2,
        ) -> vk::Result;

        pub fn vpGetProfileQueueFamilyStructureTypes(
            pProfile: *const ProfileProperties,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileFormats(
            pProfile: *const ProfileProperties,
            pFormatCount: *mut u32,
            pFormats: *mut vk::Format,
        ) -> vk::Result;

        pub fn vpGetProfileFormatProperties(
            pProfile: *const ProfileProperties,
            format: vk::Format,
            pNext: *mut c_void,
        );

        pub fn vpGetProfileFormatStructureTypes(
            pProfile: *const ProfileProperties,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;
    }
}