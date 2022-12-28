#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod arch;

// use core::{slice, ptr};
// use arch;

use log::info;
use uefi::prelude::*;
// use uefi::{prelude::*, proto::{loaded_image::LoadedImage, device_path::DevicePath, media::{fs::SimpleFileSystem, file::{FileMode, FileAttribute, File, FileInfo}}, network::{pxe::{BaseCode, DhcpV4Packet}, IpAddress}, console::gop::GraphicsOutput}, table::boot::{OpenProtocolParams, OpenProtocolAttributes, AllocateType, MemoryType}, CStr16, CStr8};

#[entry]
fn main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    log::info!("UEFI bootloader started");

    // #[cfg(feature = "logger")]
    // init_logger(st);
    info!("Hello world!");

    // let kernel = load_kernel(image, &system_table);
    // log::info!("Reading kernel and configuration from disk was successful");

    // // we no longer need the system table for printing panics
    // unsafe {
    //     SYSTEM_TABLE = None;
    // }

    // 
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}