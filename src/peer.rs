use libc::*;
use ::list::ENetList;
use ::address::ENetAddress;

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
    // TODO: Continue here
}
