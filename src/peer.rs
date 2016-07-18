use libc::*;
use ::list::{ENetList, ENetListNode};
use ::address::ENetAddress;
use ::{ENetChannel, ENetAcknowledgement, ENetIncomingCommand, ENetOutgoingCommand};
use ::host::ENetHost;
use ::protocol::ENetProtocol;
use packet::ENetPacket;

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
    pub dispatchList: ENetListNode,
    pub host: *mut ENetHost,
    pub outgoingPeerID: uint16_t,
    pub incomingPeerID: uint16_t,
    pub connectID: uint32_t,
    pub outgoingSessionID: uint8_t,
    pub incomingSessionID: uint8_t,
    pub address: ENetAddress,
    pub data: *mut c_void,
    pub state: ENetPeerState,
    pub channels: *mut ENetChannel,
    pub channelCount: size_t,
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub incomingBandwidthThrottleEpoch: uint32_t,
    pub outgoingBandwidthThrottleEpoch: uint32_t,
    pub incomingDataTotal: uint32_t,
    pub outgoingDataTotal: uint32_t,
    pub lastSendTime: uint32_t,
    pub lastReceiveTime: uint32_t,
    pub nextTimeout: uint32_t,
    pub earliestTimeout: uint32_t,
    pub packetLossEpoch: uint32_t,
    pub packetsSent: uint32_t,
    pub packetsLost: uint32_t,
    pub packetLoss: uint32_t,
    pub packetLossVariance: uint32_t,
    pub packetThrottle: uint32_t,
    pub packetThrottleLimit: uint32_t,
    pub packetThrottleCounter: uint32_t,
    pub packetThrottleEpoch: uint32_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub pingInterval: uint32_t,
    pub timeoutLimit: uint32_t,
    pub timeoutMaximum: uint32_t,
    pub timeoutMinimum: uint32_t,
    pub lastRoundTripTime: uint32_t,
    pub lowestRoundTripTime: uint32_t,
    pub lastRoundTripTimeVariance: uint32_t,
    pub highestRoundTripTimeVariance: uint32_t,
    pub roundTripTime: uint32_t,
    pub roundTripTimeVariance: uint32_t,
    pub mtu: uint32_t,
    pub windowSize: uint32_t,
    pub reliabledataInTransit: uint32_t,
    pub outgoingReliableSequenceNumber: uint16_t,
    pub acknowledgements: ENetList,
    pub sentReliableCommands: ENetList,
    pub sentUnreliableCommands: ENetList,
    pub outgoingReliableCommands: ENetList,
    pub outgoingUnreliableCommands: ENetList,
    pub dispatchCommands: ENetList,
    pub needsDispatch: c_int,
    pub incomingUnsequencedGroup: uint16_t,
    pub outgoingUnsequencedGroup: uint16_t,
    pub unsequencedWindow: [uint32_t; ENET_PEER_UNSEQUENCED_WINDOW_SIZE/32],
    pub eventData: uint32_t,
    pub totalWaitingData: size_t,
}

#[repr(C)]
#[derive(Clone,Copy,PartialEq)]
pub enum ENetPeerState {
    ENET_PEER_STATE_DISCONNECTED = 0,
    ENET_PEER_STATE_CONNECTING = 1,
    ENET_PEER_STATE_ACKNOWLEDGING_CONNECT = 2,
    ENET_PEER_STATE_CONNECTION_PENDING = 3,
    ENET_PEER_STATE_CONNECTION_SUCCEEDED = 4,
    ENET_PEER_STATE_CONNECTED = 5,
    ENET_PEER_STATE_DISCONNECT_LATER = 6,
    ENET_PEER_STATE_DISCONNECTING = 7,
    ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT = 8,
    ENET_PEER_STATE_ZOMBIE = 9,
}

extern {
    pub fn enet_peer_disconnect(peer: *mut ENetPeer, data: uint32_t);
    pub fn enet_peer_disconnect_later(peer: *mut ENetPeer, data: uint32_t);
    pub fn enet_peer_disconnect_now(peer: *mut ENetPeer, data: uint32_t);
    pub fn enet_peer_dispatch_incoming_reliable_commands(peer: *mut ENetPeer, channel: *mut ENetChannel);
    pub fn enet_peer_dispatch_incoming_unreliable_commands(peer: *mut ENetPeer, channel: *mut ENetChannel);
    pub fn enet_peer_on_connect(peer: *mut ENetPeer);
    pub fn enet_peer_on_disconnect(peer: *mut ENetPeer);
    pub fn enet_peer_ping(peer: *mut ENetPeer);
    pub fn enet_peer_ping_interval(peer: *mut ENetPeer, pingInterval: uint32_t);
    pub fn enet_peer_queue_acknowledgement(peer: *mut ENetPeer, command: *const ENetProtocol,
            sentTime: uint16_t) -> *mut ENetAcknowledgement;
    pub fn enet_peer_queue_incoming_command(peer: *mut ENetPeer, command: *const ENetProtocol,
            data: *const c_void, dataLength: size_t, flags: uint32_t, fragmentCount: uint32_t)
            -> *mut ENetIncomingCommand;
    pub fn enet_peer_queue_outgoing_command(peer: *mut ENetPeer, command: *const ENetProtocol,
            packet: *mut ENetPacket, offset: uint32_t, length: uint16_t) -> *mut ENetOutgoingCommand;
    pub fn enet_peer_receive(peer: *mut ENetPeer, channelID: *mut uint8_t) -> *mut ENetPacket;
    pub fn enet_peer_reset(peer: *mut ENetPeer);
    pub fn enet_peer_reset_queues(peer: *mut ENetPeer);
    pub fn enet_peer_send(peer: *mut ENetPeer, channelID: uint8_t, packet: *mut ENetPacket) -> c_int;
    pub fn enet_peer_setup_outgoing_command(peer: *mut ENetPeer,
            outgoingCommand: *mut ENetOutgoingCommand);
    pub fn enet_peer_throttle(peer: *mut ENetPeer, rtt: uint32_t) -> c_int;
    pub fn enet_peer_throttle_configure(peer: *mut ENetPeer, interval: uint32_t, acceleration: uint32_t,
            deceleration: uint32_t);
    pub fn enet_peer_timeout(peer: *mut ENetPeer, timeoutLimit: uint32_t, timeoutMinimum: uint32_t,
            timeoutMaximum: uint32_t);
}
