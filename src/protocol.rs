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
    acknowledge: ENetProtocolAcknowledge,
    bandwidthLimit: ENetProtocolBandwidthLimit,
    connect: ENetProtocolConnect,
    disconnect: ENetProtocolDisconnect,
    header: ENetProtocolCommandHeader,
    ping: ENetProtocolPing,
    sendFragment: ENetProtocolSendFragment,
    sendReliable: ENetProtocolSendReliable,
    sendUnreliable: ENetProtocolSendUnreliable,
    sendUnsequenced: ENetProtocolSendUnsequenced,
    throttleConfigure: ENetProtocolThrottleConfigure,
    verifyConnect: ENetProtocolVerifyConnect,
  }
}

#[repr(C)]
pub struct ENetProtocolAcknowledge {
    pub receivedReliableSequenceNumber: uint16_t,
    pub receivedSentTime: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolBandwidthLimit {
    pub incomingBandwidth: uint32_t,
    pub outgoingBandwidth: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolConnect {
    pub channelCount: uint32_t,
    pub connectID: uint32_t,
    pub data: uint32_t,
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: uint32_t,
    pub incomingSessionID: uint8_t,
    pub mtu: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub outgoingSessionID: uint8_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub windowSize: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolDisconnect {
    pub data: uint32_t,
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
pub struct ENetProtocolCommandHeader {
    pub channelID: uint8_t,
    pub command: uint8_t,
    pub reliableSequenceNumber: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
pub struct ENetProtocolSendFragment {
    pub dataLength: uint16_t,
    pub fragmentCount: uint32_t,
    pub fragmentNumber: uint32_t,
    pub fragmentOffset: uint32_t,
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: uint16_t,
    pub totalLength: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolSendReliable {
    pub dataLength: uint16_t,
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
pub struct ENetProtocolSendUnreliable {
    pub dataLength: uint16_t,
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolSendUnsequenced {
    pub dataLength: uint16_t,
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: uint16_t,
}

#[repr(C)]
pub struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub packetThrottleInterval: uint32_t,
}

#[repr(C)]
pub struct ENetProtocolVerifyConnect {
    pub channelCount: uint32_t,
    pub connectID: uint32_t,
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: uint32_t,
    pub incomingSessionID: uint8_t,
    pub mtu: uint32_t,
    pub outgoingBandwidth: uint32_t,
    pub outgoingPeerID: uint16_t,
    pub outgoingSessionID: uint8_t,
    pub packetThrottleAcceleration: uint32_t,
    pub packetThrottleDeceleration: uint32_t,
    pub packetThrottleInterval: uint32_t,
    pub windowSize: uint32_t,
}
