extern crate enet_sys;

use std::mem::size_of;

use enet_sys::*;
//use enet_sys::{enet_initialize, enet_deinitialize};
use enet_sys::address::ENetAddress;
use enet_sys::host::{enet_host_create, enet_host_destroy, ENetHost};
use enet_sys::list::ENetList;
use enet_sys::socket::ENetSocket;
use enet_sys::peer::ENetPeer;
use enet_sys::protocol::ENetProtocol;

fn main() {
    println!("size of ENetHost: {}", size_of::<ENetHost>());
    println!("size of ENetAddress: {}", size_of::<ENetAddress>());
    println!("size of ENetSocket: {}", size_of::<ENetSocket>());
    println!("size of ENetInterceptCallback: {}",
      size_of::<ENetInterceptCallback>());
    println!("size of ENetList: {}", size_of::<ENetList>());
    println!("size of ENetCompressor: {}", size_of::<ENetCompressor>());
    println!("size of ENetProtocol: {}", size_of::<ENetProtocol>());
    println!("size of ENetBuffer: {}", size_of::<ENetBuffer>());

    println!("size of ENetEvent: {}", size_of::<ENetEvent>());
    println!("size of ENetPeer: {}", size_of::<ENetPeer>());

    println!("Starting test of enet bindings...");
    if unsafe {enet_initialize()} < 0 {
        panic!("Error on enet initialization.");
    }

    println!("creating ENet server host...");

    // default localhost on port 12345
    let address = ENetAddress {
      host: 0,
      port: 12345 };

    unsafe {
      let server = enet_host_create (
        &address,  // address to bind the server host to
        32,        // allow up to 32 clients and/or outgoing connections
        2,         // allow up to 2 channels to be used, 0 and 1
        0,         // assume any amount of incoming bandwidth
        0);        // assume any amount of outgoing bandwidth
      if server.is_null() {
        panic!("error: host create returned null")
      }
      println!("...ENet server host created");

      println!("server: {:p}", server);
      println!("server peers: {:p}", server.as_ref().unwrap().peers);
      println!("server &packetData: {:p}", &(*server).packetData);
      println!("server &serviceTime: {:p}", &(*server).serviceTime);
      println!("server channelLimit: {}", (*server).channelLimit);
      println!("server peerCount: {}", server.as_ref().unwrap().peerCount);
      println!("server connectedPeers: {}", (*server).connectedPeers);
      println!("server randomSeed: {:?}", (*server).randomSeed);
      println!("server socket: {:?}", (*server).socket);

      enet_host_destroy (server);
    }

    println!("Enet initialized.");
    unsafe {enet_deinitialize();}
    println!("Enet deinitialized.");
}
