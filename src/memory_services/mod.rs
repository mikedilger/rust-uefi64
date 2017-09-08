// Copyright 2017 Michael Dilger
// Refer to LICENSE-MIT and LICENSE-APACHE files

pub mod types;
pub use self::types::*;

use types::*;

/// Allocates pages of a particular type
pub type AllocatePages = unsafe extern "win64" fn (
    type_: AllocateType,
    memory_type: MemoryType,
    pages: usize,
    // The pointer to physical memory allocated.  May be required as an input paramter
    // as well depending on `type_`
    memory: *mut PhysicalAddress) -> Status;

/// Frees allocated pages
pub type FreePages = unsafe extern "win64" fn(
    // The base physical address of the pages to be freed
    memory: PhysicalAddress,
    // The number of contiguous 4K pages to free
    pages: usize) -> Status;

/// Returns the current boot services memory map and memory map key
pub type GetMemoryMap = unsafe extern "win64" fn(
    // The size of `memory_map`.  On failure, firmware will write the size the
    // map needs to be here, so you can try again.
    memory_map_size: *mut usize,
    // An array of MemoryDescriptors, to contain the output
    memory_map: *mut MemoryDescriptor,
    // A key for the current memory map
    map_key: *mut usize,
    // The size of an individual memory descriptor
    descriptor_size: *mut usize,
    descriptor_version: *mut u32) -> Status;

// Allocates a pool of a particular type
pub type AllocatePool = unsafe extern "win64" fn(
    // The type of pool to allocate.
    pool_type: MemoryType,
    // The number of bytes to allocate from the pool.
    size: usize,
    // A pointer to a pointer to the allocated buffer if the call succeeds; undefined otherwise.
    buffer: *mut *mut () ) -> Status;

// Frees allocated pool
pub type FreePool = unsafe extern "win64" fn(
    // pointer to the buffer to free
    buffer: *mut () ) -> Status;
