// Copyright 2017 Michael Dilger
// Refer to LICENSE-MIT and LICENSE-APACHE files

//! Your UEFI application (bootloader or OS) will need an entry point (lets call it
//! "efi_main" but you can tell the linker to use any name you want) which is setup
//! like this:
//!
//! ```rust,ignore
//! use uefi64::{Handle, SystemTable, Status};
//!
//! #[no_mangle]
//! pub extern "win64" fn efi_main(image_handle: Handle, system_table: *const SystemTable)
//!     -> Status
//! {
//! }
//! ```
//!
//!

#![no_std]

#[macro_use]
extern crate bitflags;

pub mod types;

pub mod system_table;

pub mod boot_services;

mod memory_services;
pub use self::memory_services::types::*;
pub use self::memory_services::{allocate_pages, free_pages, MemoryMap, MemoryMapFailure,
                                get_memory_map};
