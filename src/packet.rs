use libc::*;

pub type ENetPacketFreeCallback = extern fn(packet: *mut ENetPacket);

#[repr(C)]
pub struct ENetPacket {
    pub data: *mut uint8_t,
    pub dataLength: size_t,
    pub flags: uint32_t,
    pub freeCallback: ENetPacketFreeCallback,
    pub referenceCount: size_t,
    pub userData: *mut c_void,
}
