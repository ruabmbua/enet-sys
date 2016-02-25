use libc::*;
use ::address::ENetAddress;
use ::{ENetBuffer, ENetChecksumCallback};
use ::protocol::{ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS, ENetProtocol};

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
    // TODO: continue here
}

extern {
    //pub fn enet_host_bandwith_limit(host: *mut ENetHost)
}
