use libc::*;

#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct ENetAddress {
    pub host: uint32_t,
    pub port: uint16_t,
}

extern {
    pub fn enet_address_get_host(address: *const ENetAddress, hostName: *mut c_char) -> c_int;
    pub fn enet_address_get_host_ip(address: *const ENetAddress, hostName: *mut c_char, nameLength: size_t) -> c_int;
    pub fn enet_address_set_host(address: *mut ENetAddress, hostName: *const c_char) -> c_int;
}
