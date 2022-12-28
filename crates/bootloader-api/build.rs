use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    // we capture the version numbers in the build script so that we can use them as consts in the actual crate
    // if `parse()` was a const fn we wouldnt need this
    let version_major: u16 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let version_minor: u16 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let version_patch: u16 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    let pre_release: bool = !env!("CARGO_PKG_VERSION_PRE").is_empty();

    fs::write(
        Path::new(&out_dir).join("version_info.rs"),
        format!(
            "
            pub const VERSION_MAJOR: u16 = {};
            pub const VERSION_MINOR: u16 = {};
            pub const VERSION_PATCH: u16 = {};
            pub const VERSION_PRE: bool = {};
            ",
            version_major, version_minor, version_patch, pre_release
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=Cargo.toml");
}