#[cfg(feature = "docs-rs")]
fn main() {}

#[cfg(not(feature = "docs-rs"))]
fn main() {
    // let dst = cmake::Config::new("libvulkanprofiles").build();

    cc::Build::new()
        .cpp(true)
        .file("libvulkanprofiles/vulkan_profiles.cpp")
        .include("libvulkanprofiles/include")
        .cpp_link_stdlib("stdc++")
        .flags(["-Wno-missing-field-initializers", "-Wno-type-limits"])
        .compile("vkprofiles");

    // println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vkprofiles");
    println!("cargo::rerun-if-changed=libvulkanprofiles/vulkan_profiles.cpp");
}
