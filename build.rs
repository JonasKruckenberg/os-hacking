use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let kernel_path = build_kernel(&out_dir);

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.join("uefi.img");
    boot_image::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
}

fn build_kernel(out_dir: &Path) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.arg("install").arg("kernel");
    // local build
    cmd.arg("--path").arg("crates/kernel");
    println!("cargo:rerun-if-changed=crates/kernel");
    cmd.arg("--locked");

    #[cfg(feature = "aarch64")]
    cmd.args(["--target", "targets/aarch64-unknown-kernel.json"]);
    #[cfg(feature = "x86_64")]
    cmd.args(["--target", "targets/x86_64-unknown-kernel.json"]);

    cmd.arg("-Zbuild-std=core")
        .arg("-Zbuild-std-features=compiler-builtins-mem");
    cmd.arg("--root").arg(out_dir);
    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd
        .status()
        .expect("failed to run cargo install for kernel");
    if status.success() {
        let path = out_dir.join("bin").join("kernel");
        assert!(
            path.exists(),
            "kernel executable does not exist after building"
        );
        path
    } else {
        panic!("failed to build kernel");
    }
}
