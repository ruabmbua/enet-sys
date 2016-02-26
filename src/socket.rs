use libc::*;
use address::ENetAddress;
use ::ENetBuffer;

pub type ENetSocket = c_int;

#[repr(C)]
pub enum ENetSocketType {
    ENET_SOCKET_TYPE_STREAM = 1,
    ENET_SOCKET_TYPE_DATAGRAM = 2,
}

#[repr(C)]
pub enum ENetSocketOption {
    ENET_SOCKOPT_NONBLOCK = 1,
    ENET_SOCKOPT_BROADCAST = 2,
    ENET_SOCKOPT_RCVBUF = 3,
    ENET_SOCKOPT_SNDBUF = 4,
    ENET_SOCKOPT_REUSEADDR = 5,
    ENET_SOCKOPT_RCVTIMEO = 6,
    ENET_SOCKOPT_SNDTIMEO = 7,
    ENET_SOCKOPT_ERROR = 8,
    ENET_SOCKOPT_NODELAY = 9,
}

#[repr(C)]
pub enum ENetSocketShutdown {
    ENET_SOCKET_SHUTDOWN_READ = 0,
    ENET_SOCKET_SHUTDOWN_WRITE = 1,
    ENET_SOCKET_SHUTDOWN_READ_WRITE = 2,
}

extern {
    pub fn enet_socket_accept(socket: ENetSocket, address: *mut ENetAddress) -> ENetSocket;
    pub fn enet_socket_bind(socket: ENetSocket, address: *const ENetAddress) -> c_int;
    pub fn enet_socket_connect(socket: ENetSocket, address: *const ENetAddress) -> c_int;
    pub fn enet_socket_create(socketType: ENetSocketType) -> ENetSocket;
    pub fn enet_socket_destroy(socket: ENetSocket);
    pub fn enet_socket_get_address(socket: ENetSocket, address: *mut ENetAddress) -> c_int;
    pub fn enet_socket_get_option(socket: ENetSocket, socketOption: ENetSocketOption,
            _unknown: *mut c_int) -> c_int;
    pub fn enet_socket_listen(socket: ENetSocket, _unknown: c_int) -> c_int;
    pub fn enet_socket_receive(socket: ENetSocket, address: *mut ENetAddress, buffer: *mut ENetBuffer,
            bufferSize: size_t) -> c_int;
    pub fn enet_socket_send(socket: ENetSocket, address: *const ENetAddress, buffer: *const ENetBuffer,
            bufferSize: size_t) -> c_int;
    pub fn enet_socket_set_option(socket: ENetSocket, socketOption: ENetSocketOption, _unknown: c_int)
            -> c_int;
    pub fn enet_socket_shutdown(socket: ENetSocket, socketShutdown: ENetSocketShutdown) -> c_int;
    pub fn enet_socket_wait(socket: ENetSocket, _unknownA: *mut uint32_t, _unknownB: uint32_t) -> c_int;
}
