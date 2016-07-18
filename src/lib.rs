#![allow(non_snake_case)]

extern crate libc;

#[macro_use]
extern crate unsafe_unions;

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
use list::{ENetList, ENetListNode};
use peer::{ENET_PEER_RELIABLE_WINDOWS, ENetPeer};
use protocol::ENetProtocol;

pub type ENetVersion = uint32_t;
pub type ENetChecksumCallback = extern fn(buffers: *const ENetBuffer, bufferCount: size_t)
        -> uint32_t;
pub type ENetInterceptCallback = extern fn(host: *mut ENetHost, event: *mut ENetEvent);

pub const ENET_HOST_ANY : uint32_t = 0;

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
    pub context: *mut c_void,
    pub compress: extern fn(context: *mut c_void, inBuffers: *const ENetBuffer,
            inBufferCount: size_t, inLimit: size_t, outData: *mut uint8_t, outLimit: size_t)
            -> size_t,
    pub decompress: extern fn(context: *mut c_void, inData: *const uint8_t, inLimit: size_t,
            outData: *mut uint8_t, outLimit: size_t) -> size_t,
    pub destroy: extern fn(context: *mut c_void),
}

#[repr(C)]
pub struct ENetEvent {
    pub _type: ENetEventType,
    pub peer: *mut ENetPeer,
    pub channelID: uint8_t,
    pub data: uint32_t,
    pub packet: *mut ENetPacket,
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
    pub outgoingReliableSequenceNumber: uint16_t,
    pub outgoingUnrelianleSequenceNumber: uint16_t,
    pub usedReliableWindows: uint16_t,
    pub reliableWindows: [uint16_t; ENET_PEER_RELIABLE_WINDOWS],
    pub incomingReliableSequenceNumber: uint16_t,
    pub incomingUnreliableSequenceNumber: uint16_t,
    pub incomingReliableCommands: ENetList,
    pub incomingUnreliableCommands: ENetList,
}

#[repr(C)]
pub struct ENetAcknowledgement {
    pub achnowledgementList: ENetListNode,
    pub sentTime: uint32_t,
    pub command: ENetProtocol,
}

#[repr(C)]
pub struct ENetIncomingCommand {
    pub incomingCommandsList: ENetListNode,
    pub reliableSequenceNumber: uint16_t,
    pub unreliableSequenceNumber: uint16_t,
    pub command: ENetProtocol,
    pub fragmentCount: uint32_t,
    pub fragmentsRemaining: uint32_t,
    pub fragments: *mut uint32_t,
    pub packet: *mut ENetPacket,
}

#[repr(C)]
pub struct ENetOutgoingCommand {
    pub outgoingCommandList: ENetListNode,
    pub reliableSequenceNumber: uint16_t,
    pub unreliableSequenceNumber: uint16_t,
    pub sentTime: uint32_t,
    pub roundTripTimeout: uint32_t,
    pub roundTripTimeoutLimit: uint32_t,
    pub fragmentOffset: uint32_t,
    pub fragmentLength: uint16_t,
    pub sendAttempts: uint16_t,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}

extern {
    pub fn enet_deinitialize();
    pub fn enet_initialize() -> c_int;
    pub fn enet_initialize_with_callbacks(version: ENetVersion, ) -> c_int;
    pub fn enet_linked_version() -> ENetVersion;
}
