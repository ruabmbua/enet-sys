extern crate libc;

pub mod address;
pub mod host;
pub mod protocol;

use libc::*;

pub type ENetVersion = uint32_t;
pub type ENetChecksumCallback = extern fn(buffers: *const ENetBuffer, bufferCount: size_t) -> uint32_t;

#[repr(C)]
pub struct ENetCallbacks {
    pub free: extern fn(memory: *mut c_void),
    pub malloc: extern fn(size: size_t) -> *mut c_void,
    pub no_memory: extern fn(),
}

#[repr(C)]
pub struct ENetBuffer {
    pub data: *mut c_void,
    pub dataLength: size_t,
}

extern {
    pub fn enet_deinitialize();
    pub fn enet_initialize() -> c_int;
    pub fn enet_initialize_with_callbacks(version: ENetVersion, ) -> c_int;
    pub fn enet_linked_version() -> ENetVersion;
}
