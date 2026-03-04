//! The vulkan profiles structures and function definitions.
//!
//! See the vulkan profiles documentation for more details <https://vulkan.lunarg.com/doc/sdk/1.3.204.1/windows/profiles_api_library.html>

use ash::vk;
use std::{ffi::c_void, ptr};

const VP_MAX_PROFILE_NAME_SIZE: usize = 256;

// todo: new vulkan_profiles API
// currently disabled (#ifdef VP_USE_OBJECT)
// #[repr(transparent)]
// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct CapabilitiesCreateFlagBits(pub(crate) vk::Flags);
// ash::vk_bitflags_wrapped!(CapabilitiesCreateFlagBits, vk::Flags);

// impl CapabilitiesCreateFlagBits {
//     pub const VP_PROFILE_CREATE_STATIC_BIT: Self = Self(1 << 0);
// }

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct CapabilitiesCreateInfo {
//     pub flags: CapabilitiesCreateFlagBits,
//     pub api_version: u32,
//     // todo: this should include a list of pointers to the Vulkan library
//     // see VpVulkanFunctions
//     // this can probably then work with Ash set to dynamic linking (loaded)
//     // for now this is just always set to nullptr
//     pub p_vulkan_functions: *const usize
// }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfileProperties {
    pub profile_name: [std::os::raw::c_char; VP_MAX_PROFILE_NAME_SIZE],
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockProperties {
    pub profiles: ProfileProperties,
    pub api_version: u32,
    pub block_name: [std::os::raw::c_char; VP_MAX_PROFILE_NAME_SIZE],
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlagBits(pub(crate) vk::Flags);
ash::vk_bitflags_wrapped!(InstanceCreateFlagBits, vk::Flags);

// nothing here
impl InstanceCreateFlagBits {}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo<'a> {
    pub p_create_info: *const vk::InstanceCreateInfo<'a>,
    pub flags: InstanceCreateFlagBits,
    pub enabled_full_profile_count: u32,
    pub p_enabled_full_profiles: *const ProfileProperties,
    pub enabled_profile_block_count: u32,
    pub p_enabled_profile_blocks: *const BlockProperties,
}
impl ::std::default::Default for InstanceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            p_create_info: ptr::null(),
            flags: InstanceCreateFlagBits::default(),
            enabled_full_profile_count: 0,
            p_enabled_full_profiles: ptr::null(),
            enabled_profile_block_count: 0,
            p_enabled_profile_blocks: ptr::null(),
        }
    }
}
impl<'a> InstanceCreateInfo<'a> {
    #[inline]
    pub fn create_info(mut self, create_info: &'a vk::InstanceCreateInfo<'a>) -> Self {
        self.p_create_info = create_info;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: InstanceCreateFlagBits) -> Self {
        self.flags = flags;
        self
    }
    pub fn enabled_full_profiles(mut self, enabled_full_profiles: &'a [ProfileProperties]) -> Self {
        self.enabled_full_profile_count = enabled_full_profiles.len() as _;
        self.p_enabled_full_profiles = enabled_full_profiles.as_ptr();
        self
    }
    pub fn enabled_profile_blocks(mut self, enabled_profile_blocks: &'a [BlockProperties]) -> Self {
        self.enabled_profile_block_count = enabled_profile_blocks.len() as _;
        self.p_enabled_profile_blocks = enabled_profile_blocks.as_ptr();
        self
    }
}

// VpCapabilities currently not used
// define_handle!(Capabilities, UNKNOWN);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceCreateFlagBits(pub(crate) vk::Flags);
ash::vk_bitflags_wrapped!(DeviceCreateFlagBits, vk::Flags);

impl DeviceCreateFlagBits {
    pub const DISABLE_ROBUST_BUFFER_ACCESS: Self = Self(0x0000001);
    pub const DISABLE_ROBUST_IMAGE_ACCESS: Self = Self(0x0000002);
    pub const DISABLE_ROBUST_ACCESS: Self = Self(0x0000001 | 0x0000002);
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo<'a> {
    pub p_create_info: *const vk::DeviceCreateInfo<'a>,
    pub flags: DeviceCreateFlagBits,
    pub enabled_full_profile_count: u32,
    pub p_enabled_full_profiles: *const ProfileProperties,
    pub enabled_profile_block_count: u32,
    pub p_enabled_profile_blocks: *const BlockProperties,
}
impl ::std::default::Default for DeviceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            p_create_info: ptr::null(),
            flags: DeviceCreateFlagBits::default(),
            enabled_full_profile_count: 0,
            p_enabled_full_profiles: ptr::null(),
            enabled_profile_block_count: 0,
            p_enabled_profile_blocks: ptr::null(),
        }
    }
}
impl<'a> DeviceCreateInfo<'a> {
    #[inline]
    pub fn create_info(mut self, create_info: &'a vk::DeviceCreateInfo<'a>) -> Self {
        self.p_create_info = create_info;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: DeviceCreateFlagBits) -> Self {
        self.flags = flags;
        self
    }
    pub fn enabled_full_profiles(mut self, enabled_full_profiles: &'a [ProfileProperties]) -> Self {
        self.enabled_full_profile_count = enabled_full_profiles.len() as _;
        self.p_enabled_full_profiles = enabled_full_profiles.as_ptr();
        self
    }
    pub fn enabled_profile_blocks(mut self, enabled_profile_blocks: &'a [BlockProperties]) -> Self {
        self.enabled_profile_block_count = enabled_profile_blocks.len() as _;
        self.p_enabled_profile_blocks = enabled_profile_blocks.as_ptr();
        self
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

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfiles = unsafe extern "C" fn(
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFallbacks = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetInstanceProfileSupport = unsafe extern "C" fn(
    pLayerName: *const std::os::raw::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const InstanceCreateInfo,
    pAllocator: *const vk::AllocationCallbacks,
    p_instance: *mut vk::Instance,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetPhysicalDeviceProfileSupport = unsafe extern "C" fn(
    instance: ash::vk::Instance,
    physicalDevice: ash::vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    supported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpCreateDevice = unsafe extern "C" fn(
    physicalDevice: ash::vk::PhysicalDevice,
    pCreateInfo: *const DeviceCreateInfo,
    pAllocator: *const vk::AllocationCallbacks,
    pDevice: *mut vk::Device,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileInstanceExtensionProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileDeviceExtensionProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFeatures = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pNext: *mut c_void,
);

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFeatureStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pNext: *mut c_void,
);

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfilePropertyStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileQueueFamilyProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::QueueFamilyProperties2,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileQueueFamilyStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFormats = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    pFormatCount: *mut u32,
    pFormats: *mut vk::Format,
) -> vk::Result;

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFormatProperties = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
    format: vk::Format,
    pNext: *mut c_void,
);

#[allow(non_camel_case_types, non_snake_case)]
pub type PFN_vpGetProfileFormatStructureTypes = unsafe extern "C" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const std::ffi::c_char,
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
            pBlockName: *const std::ffi::c_char,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::ExtensionProperties,
        ) -> vk::Result;

        pub fn vpGetProfileDeviceExtensionProperties(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::ExtensionProperties,
        ) -> vk::Result;

        pub fn vpGetProfileFeatures(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pNext: *mut c_void,
        );

        pub fn vpGetProfileFeatureStructureTypes(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileProperties(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pNext: *mut c_void,
        );

        pub fn vpGetProfilePropertyStructureTypes(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileQueueFamilyProperties(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::QueueFamilyProperties2,
        ) -> vk::Result;

        pub fn vpGetProfileQueueFamilyStructureTypes(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileFormats(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pFormatCount: *mut u32,
            pFormats: *mut vk::Format,
        ) -> vk::Result;

        pub fn vpGetProfileFormatProperties(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            format: vk::Format,
            pNext: *mut c_void,
        );

        pub fn vpGetProfileFormatStructureTypes(
            pProfile: *const ProfileProperties,
            pBlockName: *const std::ffi::c_char,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;
    }
}
