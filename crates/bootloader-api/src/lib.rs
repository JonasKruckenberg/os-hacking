#![cfg_attr(not(test), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod info;

mod version_info {
    include!(concat!(env!("OUT_DIR"), "/version_info.rs"));
}