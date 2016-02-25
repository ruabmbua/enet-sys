#[repr(C)]
pub struct ENetList {
    pub sentinel: ENetListNode,
}

#[repr(C)]
pub struct ENetListNode {
    pub next: *mut ENetListNode,
    pub previous: *mut ENetListNode,
}
