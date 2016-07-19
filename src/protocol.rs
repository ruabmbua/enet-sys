use libc::*;

pub const ENET_PROTOCOL_MINIMUM_MTU: size_t = 576;
pub const ENET_PROTOCOL_MAXIMUM_MTU: size_t = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: size_t = 32;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: size_t = 4096;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: size_t = 65536;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: size_t = 1;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: size_t = 255;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: size_t = 0xfff;
pub const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: size_t = 1024 * 1024;

#[repr(C)]
unsafe_unions!{
  pub union ENetProtocol: [u8; 48] {
    header: ENetProtocolCommandHeader,
    acknowledge: ENetProtocolAcknowledge,
    connect: ENetProtocolConnect,
    verifyConnect: ENetProtocolVerifyConnect,
    disconnect: ENetProtocolDisconnect,
    ping: ENetProtocolPing,
    sendReliable: ENetProtocolSendReliable,
    sendUnreliable: ENetProtocolSendUnreliable,
    sendUnsequenced: ENetProtocolSendUnsequenced,
    sendFragment: ENetProtocolSendFragment,
    bandwidthLimit: ENetProtocolBandwidthLimit,
    throttleConfigure: ENetProtocolThrottleConfigure,
  }
}

#[repr(C)]
pub struct ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: uint16_t,
    pub receivedSentTime: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: uint16_t,
    pub incomingSessionID: uint8_t,
    pub outgoingSessionID: uint8_t,
    pub mtu: uint32_t,
    pub windowSize: uint32_t,
    pub channelCount: uint32_t,
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub connectID: uint32_t,
    pub data: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolCommandHeader {
    pub command: uint8_t,
    pub channelID: uint8_t,
    pub reliableSequenceNumber: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolHeader {
    pub peerID: uint16_t,
    pub sendTime: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
pub struct ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: uint16_t,
    pub dataLength: uint16_t,
    pub fragmentCount: uint32_t,
    pub fragmentNumber: uint32_t,
    pub totalLength: uint32_t,
    pub fragmentOffset: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub dataLength: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: uint16_t,
    pub dataLength: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: uint16_t,
    pub dataLength: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: uint32_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: uint16_t,
    pub incomingSessionID: uint8_t,
    pub outgoingSessionID: uint8_t,
    pub mtu: uint32_t,
    pub windowSize: uint32_t,
    pub channelCount: uint32_t,
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub connectID: uint32_t,
}
