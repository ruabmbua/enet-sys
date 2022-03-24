#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(unaligned_references)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Items, that could not be generated with bindgen, because they involentarily create valid doctests in their comments.

#[doc = " ENet packet structure."]
#[doc = ""]
#[doc = " An ENet data packet that may be sent to or received from a peer. The shown"]
#[doc = " fields should only be read and never modified. The data field contains the"]
#[doc = " allocated data for the packet. The dataLength fields specifies the length"]
#[doc = " of the allocated data.  The flags field is either 0 (specifying no flags),"]
#[doc = " or a bitwise-or of any combination of the following flags:"]
#[doc = ""]
#[doc = "  ENET_PACKET_FLAG_RELIABLE - packet must be received by the target peer"]
#[doc = "  and resend attempts should be made until the packet is delivered"]
#[doc = ""]
#[doc = "  ENET_PACKET_FLAG_UNSEQUENCED - packet will not be sequenced with other packets"]
#[doc = "  (not supported for reliable packets)"]
#[doc = ""]
#[doc = "  ENET_PACKET_FLAG_NO_ALLOCATE - packet will not allocate data, and user must supply it instead"]
#[doc = ""]
#[doc = "  ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT - packet will be fragmented using unreliable"]
#[doc = "  (instead of reliable) sends if it exceeds the MTU"]
#[doc = ""]
#[doc = "  ENET_PACKET_FLAG_SENT - whether the packet has been sent from all queues it has been entered into"]
#[doc = "@sa ENetPacketFlag"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ENetPacket {
    #[doc = "< internal use only"]
    pub referenceCount: size_t,
    #[doc = "< bitwise-or of ENetPacketFlag constants"]
    pub flags: enet_uint32,
    #[doc = "< allocated data for packet"]
    pub data: *mut enet_uint8,
    #[doc = "< length of data"]
    pub dataLength: size_t,
    #[doc = "< function to be called when the packet is no longer in use"]
    pub freeCallback: ENetPacketFreeCallback,
    #[doc = "< application private data, may be freely modified"]
    pub userData: *mut ::std::os::raw::c_void,
}

pub type ENetPacket = _ENetPacket;
