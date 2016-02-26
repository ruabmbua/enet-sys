extern crate enet_sys;

use enet_sys::{enet_initialize, enet_deinitialize};

fn main() {
    println!("Starting test of enet bindings...");
    if unsafe {enet_initialize()} < 0 {
        panic!("Error on enet initialization.");
    }
    println!("Enet initialized.");
    unsafe {enet_deinitialize();}
    println!("Enet deinitialized.");
}
