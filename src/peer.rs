use libc::*;
use ::list::{ENetList, ENetListNode};
use ::address::ENetAddress;
use ::ENetChannel;
use ::host::ENetHost;

pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: size_t = 500;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: size_t = 32;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: size_t = 32;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: size_t = 7;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: size_t = 2;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: size_t = 2;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: size_t = 5000;
pub const ENET_PEER_PACKET_LOSS_SCALE: size_t = 1<<16;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: size_t = 10000;
pub const ENET_PEER_WINDOW_SIZE_SCALE: size_t = 64 * 1024;
pub const ENET_PEER_TIMEOUT_LIMIT: size_t = 32;
pub const ENET_PEER_TIMEOUT_MINIMUM: size_t = 5000;
pub const ENET_PEER_TIMEOUT_MAXIMUM: size_t = 30000;
pub const ENET_PEER_PING_INTERVAL: size_t = 500;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: size_t = 64;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: size_t = 1024;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: size_t = 32;
pub const ENET_PEER_RELIABLE_WINDOWS: size_t = 16;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: size_t = 0x1000;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: size_t = 8;

#[repr(C)]
pub struct ENetPeer {
    pub acknowledgements: ENetList,
    pub address: ENetAddress,
    pub channelCount: size_t,
    pub channels: *mut ENetChannel,
    pub connectID: uint32_t,
    pub data: *mut c_void,
    pub dispatchCommands: ENetList,
    pub dispatchList: ENetListNode,
    pub earliestTimeout: uint32_t,
    pub eventData: uint32_t,
    pub highestRoundTripTimeVariance: uint32_t,
    pub host: *mut ENetHost,
    pub incomingBandwidth: uint32_t,
    pub incomingBandwidthThrottleEpoch: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub outgoingBandwidthThrottleEpoch: uint32_t,
    pub outgoingDataTotal: uint32_t,
    pub outgoingPeerID: uint16_t,
    pub outgoingReliableCommands: ENetList,
    pub outgoingReliableSequenceNumber: uint16_t,
    pub outgoingSessionID: uint8_t,
    pub outgoingUnreliableCommands: ENetList,
    pub outgoingUnsequencedGroup: uint16_t,
    pub packetLoss: uint32_t,
    pub packetLossEpoch: uint32_t,
    pub packetLossVariance: uint32_t,
    pub packetLost: uint32_t,
    pub packetSent: uint32_t,
    pub packetThrottle: uint32_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleCounter: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub packetThrottleEpoch: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub packetThrottleLimit: uint32_t,
    pub pingInterval: uint32_t,
    pub reliabledataInTransit: uint32_t,
    pub roundTripTimeVariance: uint32_t,
    pub sentReliableCommands: ENetList,
    pub sentUnreliableCommands: ENetList,
    pub state: ENetPeerState,
    pub timeoutLimit: uint32_t,
    pub timeoutMaximum: uint32_t,
    pub timeoutMinimum: uint32_t,
    pub totalWaitingData: size_t,
    pub unsequencedWindows: [uint32_t; ENET_PEER_UNSEQUENCED_WINDOW_SIZE/32],
    pub windowSize: uint32_t,
}

#[repr(C)]
pub enum ENetPeerState {
    ENET_PEER_STATE_DISCONNECTED = 0,
    ENET_PEER_STATE_CONNECTING = 1,
    ENET_PEER_STATE_ACKNOWLEDGING_CONNECT = 2,
    ENET_PEER_STATE_CONNECTION_PENDING = 3,
    ENET_PEER_STATE_CONNECTION_SUCCEEDED = 4,
    ENET_PEER_STATE_CONNECTED = 5,
    ENET_PEER_STATE_DISCONNECT_LATER = 6,
    ENET_PEER_STATE_DISCONNECTING = 7,
    ENET_PEER_STATE_ACHNOWLEDGING_DISCONNECT = 8,
    ENET_PEER_STATE_ZOMBIE = 9,
}

extern {
    // TODO: Continue here.
}
