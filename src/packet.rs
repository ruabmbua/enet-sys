use libc::*;
use ::ENetBuffer;

pub type ENetPacketFreeCallback = extern fn(packet: *mut ENetPacket);

#[repr(C)]
pub struct ENetPacket {
    pub referenceCount: size_t,
    pub flags: uint32_t,
    pub data: *mut uint8_t,
    pub dataLength: size_t,
    pub freeCallback: ENetPacketFreeCallback,
    pub userData: *mut c_void,
}

#[repr(C)]
pub enum ENetPacketFlag {
    ENET_PACKET_FLAG_RELIABLE = 1,
    ENET_PACKET_FLAG_UNSEQUENCED = 1<<1,
    ENET_PACKET_FLAG_NO_ALLOCATE = 1<<2,
    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT = 1<<3,
    ENET_PACKET_FLAG_SENT = 1<<8,
}

extern {
    pub fn enet_crc32(buffers: *const ENetBuffer, bufferCount: size_t) -> uint32_t;
    pub fn enet_packet_create(data: *const c_void, dataLength: size_t, flags: uint32_t)
            -> *mut ENetPacket;
    pub fn enet_packet_destroy(packet: *mut ENetPacket);
    pub fn enet_packet_resize(packet: *mut ENetPacket, dataLength: size_t) -> c_int;
}
