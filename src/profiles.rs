use std::ffi::CStr;

use ash::vk;

use crate::{prelude::c_char_array_from_cstr, vp::ProfileProperties};

macro_rules! profile {
    ($name_ident:ident, $name_lib:expr, $spec:expr, $min_api:expr) => {
        pub struct $name_ident;
        impl $name_ident {
            pub const NAME: &CStr = $name_lib;
            pub const SPEC_VERSION: u32 = $spec;
            pub const MIN_API_VERSION: u32 = $min_api;

            pub const fn profile_properties() -> ProfileProperties {
                ProfileProperties {
                    profile_name: c_char_array_from_cstr(Self::NAME).unwrap(),
                    spec_version: Self::SPEC_VERSION,
                }
            }
        }
    };
}

profile!(
    AndroidBaseline2022,
    c"VP_ANDROID_baseline_2022",
    2,
    vk::make_api_version(0, 1, 1, 106)
);
profile!(
    Android15Minimums,
    c"VP_ANDROID_15_minimums",
    1,
    vk::make_api_version(0, 1, 3, 273)
);
profile!(
    Android16Minimums,
    c"VP_ANDROID_16_minimums",
    1,
    vk::make_api_version(0, 1, 3, 276)
);
profile!(
    AndroidBaseline2021,
    c"VP_ANDROID_baseline_2021",
    3,
    vk::make_api_version(0, 1, 0, 68)
);
profile!(
    KhrRoadmap2022,
    c"VP_KHR_roadmap_2022",
    1,
    vk::make_api_version(0, 1, 3, 204)
);
profile!(
    KhrRoadmap2024,
    c"VP_KHR_roadmap_2024",
    1,
    vk::make_api_version(0, 1, 3, 276)
);
profile!(
    LunargDesktopBaseline2022,
    c"VP_LUNARG_desktop_baseline_2022",
    2,
    vk::make_api_version(0, 1, 1, 139)
);
profile!(
    LunargDesktopBaseline2023,
    c"VP_LUNARG_desktop_baseline_2023",
    2,
    vk::make_api_version(0, 1, 2, 148)
);
profile!(
    LunargDesktopBaseline2024,
    c"VP_LUNARG_desktop_baseline_2024",
    1,
    vk::make_api_version(0, 1, 2, 197)
);
profile!(
    LunargMinimumRequirements1_0,
    c"VP_LUNARG_minimum_requirements_1_0",
    1,
    vk::make_api_version(0, 1, 0, 68)
);
profile!(
    LunargMinimumRequirements1_1,
    c"VP_LUNARG_minimum_requirements_1_1",
    1,
    vk::make_api_version(0, 1, 1, 108)
);
profile!(
    LunargMinimumRequirements1_2,
    c"VP_LUNARG_minimum_requirements_1_2",
    1,
    vk::make_api_version(0, 1, 2, 131)
);
profile!(
    LunargMinimumRequirements1_3,
    c"VP_LUNARG_minimum_requirements_1_3",
    1,
    vk::make_api_version(0, 1, 3, 204)
);
