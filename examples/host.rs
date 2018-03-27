extern crate enet_sys;

use enet_sys::{enet_initialize, enet_deinitialize};
use enet_sys::ENetAddress;
use enet_sys::{enet_host_create, enet_host_destroy};

fn main() {
    println!("Starting test of host creation...");
    if unsafe {enet_initialize()} < 0 {
        panic!("Error on enet initialization.");
    }
    println!("Enet initialized.");

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

    unsafe {enet_deinitialize();}
    println!("Enet deinitialized.");
}
