use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let uefi_path = build_uefi_bootloader(&out_dir);
    println!(
        "cargo:rustc-env=UEFI_BOOTLOADER_PATH={}",
        uefi_path.display()
    );
}

fn build_uefi_bootloader(out_dir: &Path) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.arg("install").arg("bootloader");
    // local build
    cmd.arg("--path").arg("../bootloader");
    println!("cargo:rerun-if-changed=../bootloader");
    cmd.arg("--locked");

    #[cfg(feature = "aarch64")]
    cmd.args(["--target", "aarch64-unknown-uefi"]);
    #[cfg(feature = "x86_64")]
    cmd.args(["--target", "x86_64-unknown-uefi"]);
    
    cmd.arg("-Zbuild-std=core")
        .arg("-Zbuild-std-features=compiler-builtins-mem");
    cmd.arg("--root").arg(out_dir);
    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd
        .status()
        .expect("failed to run cargo install for uefi bootloader");
    if status.success() {
        let path = out_dir.join("bin").join("bootloader.efi");
        assert!(
            path.exists(),
            "uefi bootloader executable does not exist after building"
        );
        path
    } else {
        panic!("failed to build uefi bootloader");
    }
}
