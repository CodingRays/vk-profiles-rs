#[cfg(feature = "docs-rs")]
fn main() {}

const VULKAN_PROFILES_COMMIT: &'static str = "0fccc7ba443a4611873ad3ad165bda5e074de344";

#[cfg(not(feature = "docs-rs"))]
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let profiles_dir = std::path::Path::new(&out_dir).join("Vulkan-Profiles-Git");

    // The Vulkan Profiles library performs some operations in the source tree.
    // Unfortunately a build.rs file is not allowed to perform operations
    // outside of OUT_DIR so we have to clone the Vulkan Profiles library into
    // OUT_DIR.
    let repo = match git2::Repository::open(&profiles_dir) {
        Ok(repo) => repo,
        Err(_) => git2::Repository::clone(
            "https://github.com/KhronosGroup/Vulkan-Profiles.git",
            &profiles_dir,
        )
        .expect("Failed to clone Vulkan-Profiles repository"),
    };

    let object = repo.revparse_single(VULKAN_PROFILES_COMMIT).unwrap();
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout Vulkan-Profiles");
    repo.set_head_detached(object.id())
        .expect("Failed to update Vulkan-Profiles HEAD");

    // generate the Vulkan-Profiles c++ files and headers
    cmake::Config::new(&profiles_dir)
        .define("UPDATE_DEPS", "ON")
        .generator("Ninja")
        .build();

    // compile and add the files as a library
    let dst = cmake::Config::new(".")
        .define(
            "VK_PROFILES_SRC_DIR",
            profiles_dir.canonicalize().unwrap()
        )
        .generator("Ninja")
        .build();

    // link the library
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vkprofiles");

    println!("cargo::rerun-if-changed=CMakeLists.txt");
}
