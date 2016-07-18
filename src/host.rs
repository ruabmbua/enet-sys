use libc::*;
use ::address::ENetAddress;
use ::{ENetBuffer, ENetChecksumCallback, ENetCompressor, ENetInterceptCallback, ENetEvent};
use ::protocol::{ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS, ENetProtocol, ENET_PROTOCOL_MAXIMUM_MTU};
use ::list::ENetList;
use ::peer::ENetPeer;
use ::socket::ENetSocket;
use ::packet::ENetPacket;

pub const ENET_HOST_RECEIVE_BUFFER_SIZE: size_t = 256 * 1024;
pub const ENET_HOST_SEND_BUFFER_SIZE: size_t = 256 * 1024;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: size_t = 1000;
pub const ENET_HOST_DEFAULT_MTU: size_t = 1400;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: size_t = 32 * 1024 * 1024;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: size_t = 32 * 1024 * 1024;

#[repr(C)]
pub struct ENetHost {
    pub socket: ENetSocket,
    pub address: ENetAddress,
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub bandwidthThrottleEpoch: uint32_t,
    pub mtu: uint32_t,
    pub randomSeed: uint32_t,
    pub recalculateBandwidthLimits: c_int,
    pub peers: *mut ENetPeer,
    pub peerCount: size_t,
    pub channelLimit: size_t,
    pub serviceTime: uint32_t,
    pub dispatchQueue: ENetList,
    pub continueSending: c_int,
    pub packetSize: size_t,
    pub headerFlags: uint16_t,
    pub commands: [ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
    pub commandCount: size_t,
    pub buffers: [ENetBuffer; 1+2 * ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
    pub bufferCount: size_t,
    pub checksum: ENetChecksumCallback,
    pub compressor: ENetCompressor,
    pub packetData: [[uint8_t; ENET_PROTOCOL_MAXIMUM_MTU]; 2],
    pub receivedAddress: ENetAddress,
    pub receivedData: *mut uint8_t,
    pub receivedDataLength: size_t,
    pub totalSentData: uint32_t,
    pub totalSentPackets: uint32_t,
    pub totalReceivedData: uint32_t,
    pub totalReceivedPackets: uint32_t,
    pub intercept: ENetInterceptCallback,
    pub connectedPeers: size_t,
    pub bandwidthLimitedPeers: size_t,
    pub duplicatePeers: size_t,
    pub maximumPacketSize: size_t,
    pub maximumWaitingData: size_t,
}

extern {
    pub fn enet_host_bandwidth_limit(host: *mut ENetHost, incomingBandwidth: uint32_t,
            outgoingBandwidth: uint32_t);
    pub fn enet_host_bandwidth_throttle(host: *mut ENetHost);
    pub fn enet_host_broadcast(host: *mut ENetHost, channelID: uint8_t, packet: *mut ENetPacket);
    pub fn enet_host_channel_limit(host: *mut ENetHost, channelLimit: size_t);
    pub fn enet_host_check_events(host: *mut ENetHost, event: *mut ENetEvent) -> c_int;
    pub fn enet_host_compress(host: *mut ENetHost, compressor: *const ENetCompressor);
    pub fn enet_host_compress_with_range_coder(host: *mut ENetHost) -> c_int;
    pub fn enet_host_connect(host: *mut ENetHost, address: *const ENetAddress, channelCount: size_t,
            data: uint32_t) -> *mut ENetPeer;
    pub fn enet_host_create(address: *const ENetAddress, peerCount: size_t, channelLimit: size_t,
            incomingBandwidth: uint32_t, outgoingBandwidth: uint32_t) -> *mut ENetHost;
    pub fn enet_host_destroy(host: *mut ENetHost);
    pub fn enet_host_flush(host: *mut ENetHost);
    pub fn enet_host_service(host: *mut ENetHost, event: *mut ENetEvent, timeout: uint32_t) -> c_int;
}
