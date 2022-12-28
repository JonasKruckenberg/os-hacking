use std::{
    env,
    process::Command,
};

#[cfg(all(feature = "aarch64", target_os = "macos" ))]
const EDK2: &str = "/opt/homebrew/Cellar/qemu/7.2.0/share/qemu/edk2-aarch64-code.fd";
#[cfg(all(feature = "x86_64", target_os = "macos" ))]
const EDK2: &str = "/opt/homebrew/Cellar/qemu/7.2.0/share/qemu/edk2-x86_64-code.fd";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uefi_path = env!("UEFI_PATH");

    #[cfg(feature = "aarch64")]
    let mut cmd = {
        let mut cmd = Command::new("qemu-system-aarch64");
        cmd.args([
            "-bios",
            EDK2,
            "-drive",
            &format!("format=raw,file={}", uefi_path),
            "-M",
            "virt",
            "-accel",
            "hvf",
            "-serial",
            "stdio",
            "-cpu",
            "max",
        ]);
        cmd
    };

    #[cfg(feature = "x86_64")]
    let mut cmd = {
        let mut cmd = Command::new("qemu-system-x86_64");
        cmd.args([
            "-drive",
            &format!("if=pflash,format=raw,unit=0,file={},readonly=on", EDK2),
            "-drive",
            &format!("format=raw,file={}", uefi_path),
            "-serial",
            "stdio",
        ]);
        cmd
    };

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
