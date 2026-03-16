use crate::prelude::*;
use crate::vp::*;

use ash::vk;

impl std::fmt::Debug for InstanceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}

impl std::fmt::Debug for DeviceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[
            (
                DeviceCreateFlagBits::DISABLE_ROBUST_BUFFER_ACCESS.0,
                "DISABLE_ROBUST_BUFFER_ACCESS",
            ),
            (
                DeviceCreateFlagBits::DISABLE_ROBUST_IMAGE_ACCESS.0,
                "DISABLE_ROBUST_IMAGE_ACCESS",
            ),
            (
                DeviceCreateFlagBits::DISABLE_ROBUST_ACCESS.0,
                "DISABLE_ROBUST_ACCESS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
