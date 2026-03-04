# VkProfiles RS

Rust bindings for the [Vulkan Profiles](https://github.com/KhronosGroup/Vulkan-Profiles) library using [Ash](https://github.com/ash-rs/ash). The crate is designed as close as possible to Ash to allow for easy usage.

**Note: The vulkan loader must be statically linked in ash**

[![Crates.io Version](https://img.shields.io/crates/v/vk-profiles-rs.svg)](https://crates.io/crates/vk-profiles-rs)
[![Documentation](https://docs.rs/vk-profiles-rs/badge.svg)](https://docs.rs/vk-profiles-rs)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)

## Additional dependencies

These are required to build the Vulkan profiles library
- A C++ compiler
- Vulkan SDK. Only the Vulkan headers are required for the build.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.