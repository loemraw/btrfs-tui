use crate::structs::{CSUM_SIZE, FSID_SIZE, UUID_SIZE};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Header {
    pub csum: [u8; CSUM_SIZE],
    pub fsid: [u8; FSID_SIZE],
    /// Which block this node is supposed to live in
    pub bytenr: u64,
    pub flags: u64,
    pub chunk_tree_uuid: [u8; UUID_SIZE],
    pub generation: u64,
    pub owner: u64,
    pub nritems: u32,
    pub level: u8,
}
