use std::{path::{Path, PathBuf}, collections::BTreeMap};

use anyhow::Context;
use tempfile::NamedTempFile;

mod fat;
mod gpt;
mod pxe;

#[cfg(feature = "aarch64")]
pub const KERNEL_FILE_NAME: &str = "kernel-aarch64";
#[cfg(feature = "x86_64")]
pub const KERNEL_FILE_NAME: &str = "kernel-x86_64";

/// Create disk images for booting on UEFI systems.
pub struct UefiBoot {
    kernel: PathBuf,
}

impl UefiBoot {
    /// Start creating a disk image for the given bootloader ELF executable.
    pub fn new(kernel_path: &Path) -> Self {
        Self {
            kernel: kernel_path.to_owned(),
        }
    }

    /// Create a bootable BIOS disk image at the given path.
    pub fn create_disk_image(&self, out_path: &Path) -> anyhow::Result<()> {
        let fat_partition = self
            .create_fat_partition()
            .context("failed to create FAT partition")?;

        gpt::create_gpt_disk(fat_partition.path(), out_path)
            .context("failed to create UEFI GPT disk image")?;

        fat_partition
            .close()
            .context("failed to delete FAT partition after disk image creation")?;

        Ok(())
    }

    /// Prepare a folder for use with booting over UEFI_PXE.
    ///
    /// This places the bootloader executable under the path "bootloader". The
    /// DHCP server should set the filename option to that path, otherwise the
    /// bootloader won't be found.
    pub fn create_pxe_tftp_folder(&self, out_path: &Path) -> anyhow::Result<()> {
        let bootloader_path = Path::new(env!("UEFI_BOOTLOADER_PATH"));

        pxe::create_uefi_tftp_folder(bootloader_path, self.kernel.as_path(), out_path)
            .context("failed to create UEFI PXE tftp folder")?;

        Ok(())
    }

    /// Creates an UEFI-bootable FAT partition with the kernel.
    fn create_fat_partition(&self) -> anyhow::Result<NamedTempFile> {
        let bootloader_path = Path::new(env!("UEFI_BOOTLOADER_PATH"));
        
        let mut files = BTreeMap::new();
        
        #[cfg(feature = "aarch64")]
        files.insert("efi/boot/bootaa64.efi", bootloader_path);
        #[cfg(feature = "x86_64")]
        files.insert("efi/boot/bootx64.efi", bootloader_path);
        
        files.insert(KERNEL_FILE_NAME, self.kernel.as_path());

        let out_file = NamedTempFile::new().context("failed to create temp file")?;
        fat::create_fat_filesystem(files, out_file.path())
            .context("failed to create UEFI FAT filesystem")?;

        Ok(out_file)
    }
}