extern crate libc;

pub mod address;
pub mod host;
pub mod protocol;
pub mod list;
pub mod packet;
pub mod peer;
pub mod socket;

use libc::*;
use host::ENetHost;
use packet::ENetPacket;
use list::ENetList;
use peer::{ENET_PEER_RELIABLE_WINDOWS, ENetPeer};

pub type ENetVersion = uint32_t;
pub type ENetChecksumCallback = extern fn(buffers: *const ENetBuffer, bufferCount: size_t)
        -> uint32_t;
pub type ENetInterceptCallback = extern fn(host: *mut ENetHost, event: *mut ENetEvent);

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

#[repr(C)]
pub struct ENetCompressor {
    pub compress: extern fn(context: *mut c_void, inBuffers: *const ENetBuffer,
            inBufferCount: size_t, inLimit: size_t, outData: *mut uint8_t, outLimit: size_t)
            -> size_t,
    pub context: *mut c_void,
    pub decompress: extern fn(context: *mut c_void, inData: *const uint8_t, inLimit: size_t,
            outData: *mut uint8_t, outLimit: size_t) -> size_t,
    pub destroy: extern fn(context: *mut c_void),
}

#[repr(C)]
pub struct ENetEvent {
    pub channelID: uint8_t,
    pub data: uint32_t,
    pub packet: *mut ENetPacket,
    pub peer: *mut ENetPeer,
    pub _type: ENetEventType,
}

#[repr(C)]
pub enum ENetEventType {
    ENET_EVENT_TYPE_NONE = 0,
    ENET_EVENT_TYPE_CONNECT = 1,
    ENET_EVENT_TYPE_DISCONNECT = 2,
    ENET_EVENT_TYPE_RECEIVE = 3,
}

#[repr(C)]
pub struct ENetChannel {
    pub incomingReliableCommands: ENetList,
    pub incomingReliableSequenceNumber: uint16_t,
    pub incomingUnreliableCommands: ENetList,
    pub incomingUnreliableSequenceNumber: uint16_t,
    pub outgoingReliableSequenceNumber: uint16_t,
    pub outgoingUnrelianleSequenceNumber: uint16_t,
    pub reliableWindows: [uint16_t; ENET_PEER_RELIABLE_WINDOWS],
    pub usedReliableWindows: uint16_t,
}

extern {
    pub fn enet_deinitialize();
    pub fn enet_initialize() -> c_int;
    pub fn enet_initialize_with_callbacks(version: ENetVersion, ) -> c_int;
    pub fn enet_linked_version() -> ENetVersion;
}
