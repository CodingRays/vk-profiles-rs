use crate::prelude::*;
use crate::vp::*;

use ash::vk;

impl std::fmt::Debug for InstanceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[
            (
                InstanceCreateFlagBits::MERGE_EXTENSIONS.0,
                "MERGE_EXTENSIONS",
            ),
            (
                InstanceCreateFlagBits::OVERRIDE_EXTENSIONS.0,
                "OVERRIDE_EXTENSIONS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}

impl std::fmt::Debug for DeviceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[
            (
                DeviceCreateFlagBits::MERGE_EXTENSIONS.0,
                "MERGE_EXTENSIONS",
            ),
            (
                DeviceCreateFlagBits::OVERRIDE_EXTENSIONS.0,
                "OVERRIDE_EXTENSIONS",
            ),
            (
                DeviceCreateFlagBits::OVERRIDE_FEATURES.0,
                "OVERRIDE_FEATURES",
            ),
            (
                DeviceCreateFlagBits::OVERRIDE_ALL_FEATURES.0,
                "OVERRIDE_ALL_FEATURES",
            ),
            (
                DeviceCreateFlagBits::DISABLE_ROBUST_BUFFER_ACCESS.0,
                "DISABLE_ROBUST_BUFFER_ACCESS",
            ),
            (
                DeviceCreateFlagBits::DISABLE_ROBUST_IMAGE_ACCESS.0,
                "DISABLE_ROBUST_BUFFER_ACCESS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}