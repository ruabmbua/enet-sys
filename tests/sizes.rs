use std::mem::size_of;

extern crate enet_sys;

use enet_sys::*;
use enet_sys::address::*;
use enet_sys::host::*;
use enet_sys::list::*;
use enet_sys::packet::*;
use enet_sys::peer::*;
use enet_sys::protocol::*;
use enet_sys::socket::*;

#[test]
fn test () {
  // lib.rs
  assert_eq!(size_of::<ENetAcknowledgement>(), 72);
  assert_eq!(size_of::<ENetBuffer>(), 16);
  assert_eq!(size_of::<ENetCallbacks>(), 24);
  assert_eq!(size_of::<ENetChannel>(), 80);
  assert_eq!(size_of::<ENetCompressor>(), 32);
  assert_eq!(size_of::<ENetIncomingCommand>(), 96);
  assert_eq!(size_of::<ENetOutgoingCommand>(), 96);

  // address.rs
  assert_eq!(size_of::<ENetAddress>(), 8);

  // host.rs
  assert_eq!(size_of::<ENetHost>(), 11024);

  // list.rs
  assert_eq!(size_of::<ENetList>(), 16);
  assert_eq!(size_of::<ENetListNode>(), 16);

  // packet.rs
  assert_eq!(size_of::<ENetPacket>(), 48);

  // peer.rs
  assert_eq!(size_of::<ENetPeer>(), 472);

  // protocol.rs
  assert_eq!(size_of::<ENetProtocol>(), 48);
  assert_eq!(size_of::<ENetProtocolAcknowledge>(), 8);
  assert_eq!(size_of::<ENetProtocolBandwidthLimit>(), 12);
  assert_eq!(size_of::<ENetProtocolCommandHeader>(), 4);
  assert_eq!(size_of::<ENetProtocolConnect>(), 48);
  assert_eq!(size_of::<ENetProtocolDisconnect>(), 8);
  assert_eq!(size_of::<ENetProtocolHeader>(), 4);
  assert_eq!(size_of::<ENetProtocolPing>(), 4);
  assert_eq!(size_of::<ENetProtocolSendFragment>(), 24);
  assert_eq!(size_of::<ENetProtocolSendReliable>(), 6);
  assert_eq!(size_of::<ENetProtocolSendUnreliable>(), 8);
  assert_eq!(size_of::<ENetProtocolSendUnsequenced>(), 8);
  assert_eq!(size_of::<ENetProtocolThrottleConfigure>(), 16);
  assert_eq!(size_of::<ENetProtocolVerifyConnect>(), 44);

  // socket.rs
  assert_eq!(size_of::<ENetSocket>(), 4);
  assert_eq!(size_of::<ENetSocketOption>(), 4);
  assert_eq!(size_of::<ENetSocketShutdown>(), 4);
  assert_eq!(size_of::<ENetSocketType>(), 4);
}
