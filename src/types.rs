// Copyright 2017 Michael Dilger
// Refer to LICENSE-MIT and LICENSE-APACHE files

#[repr(C)] // align 64-bit
pub struct Guid(pub u64, pub u64);

pub type Status = usize;

pub type Handle = *mut ();

pub type Event = *mut ();

pub type Lba = u64;

pub type Tpl = usize;

#[repr(C)]
pub struct MacAddress(pub [u8; 32]);

#[repr(C)]
pub struct Ipv4Address(pub [u8; 4]);

#[repr(C)]
pub struct Ipv6Address(pub [u8; 16]);

#[repr(C)] // align 4-byte
pub struct IpAddress(pub [u8; 16]);

// for enums, use #[repr(u32)]
