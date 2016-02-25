use libc::*;
use ::address::ENetAddress;
use ::{ENetBuffer, ENetChecksumCallback, ENetCompressor, ENetInterceptCallback};
use ::protocol::{ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS, ENetProtocol, ENET_PROTOCOL_MAXIMUM_MTU};
use ::list::ENetList;
use ::peer::ENetPeer;
use ::socket::ENetSocket;

pub const ENET_HOST_RECEIVE_BUFFER_SIZE: size_t = 256 * 1024;
pub const ENET_HOST_SEND_BUFFER_SIZE: size_t = 256 * 1024;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: size_t = 1000;
pub const ENET_HOST_DEFAULT_MTU: size_t = 1400;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: size_t = 32 * 1024 * 1024;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: size_t = 32 * 1024 * 1024;

#[repr(C)]
pub struct ENetHost {
    pub address: ENetAddress,
    pub bandwidthLimitedPeers: size_t,
    pub bandwidthThrottleEpoch: uint32_t,
    pub bufferCount: size_t,
    pub buffers: [ENetBuffer; 1+2 * ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
    pub channelLimit: size_t,
    pub checksum: ENetChecksumCallback,
    pub commandCount: size_t,
    pub commands: [ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
    pub compressor: ENetCompressor,
    pub connectedPeers: size_t,
    pub continueSending: c_int,
    pub dispatchQueue: ENetList,
    pub duplicatePeers: size_t,
    pub headerFlags: uint16_t,
    pub incomingBandwidth: uint32_t,
    pub intercept: ENetInterceptCallback,
    pub maximumPacketSize: size_t,
    pub maximumWaitingData: size_t,
    pub mtu: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub packetData: [[uint8_t; ENET_PROTOCOL_MAXIMUM_MTU]; 2],
    pub packetSize: size_t,
    pub peerCount: size_t,
    pub peers: *mut ENetPeer,
    pub randomSeed: uint32_t,
    pub recalculateBandwidthLimits: c_int,
    pub receivedAddress: ENetAddress,
    pub receivedData: *mut uint8_t,
    pub receivedDataLength: size_t,
    pub serviceTime: uint32_t,
    pub socket: ENetSocket,
    pub totalReceivedData: uint32_t,
    pub totalReceivedPackets: uint32_t,
    pub totalSentData: uint32_t,
    pub totalSentPackets: uint32_t,
}

extern {
    pub fn enet_host_bandwidth_limit(host: *mut ENetHost, incomingBandwidth: uint32_t,
            outgoingBandwidth: uint32_t);
}
