#[cfg(feature = "docs-rs")]
fn main() {}

#[cfg(not(feature = "docs-rs"))]
fn main() {
    // generate the Vulkan-Profiles c++ files and headers
    cmake::Config::new("Vulkan-Profiles")
        .define("UPDATE_DEPS", "ON")
        .out_dir("Vulkan-Profiles/build")
        .build();

    // compile and add the files as a library
    let dst = cmake::Config::new(".").build();

    // link the library
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vkprofiles");

    println!("cargo::rerun-if-changed=Vulkan-Profiles/CMakeLists.txt");
}
