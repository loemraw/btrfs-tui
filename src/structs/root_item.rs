use crate::structs::InodeItem;
use crate::structs::Key;
use crate::structs::Timespec;
use crate::structs::UUID_SIZE;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RootItem {
    pub inode: InodeItem,
    pub generation: u64,
    pub root_dirid: u64,
    pub bytenr: u64,
    pub byte_limit: u64,
    pub bytes_used: u64,
    pub last_snapshot: u64,
    pub flags: u64,
    pub refs: u32,
    pub drop_progress: Key,
    pub drop_level: u8,
    pub level: u8,
    pub generation_v2: u64,
    pub uuid: [u8; UUID_SIZE],
    pub parent_uuid: [u8; UUID_SIZE],
    pub received_uuid: [u8; UUID_SIZE],
    /// updated when an inode changes
    pub ctransid: u64,
    /// trans when created
    pub otransid: u64,
    /// trans when sent. non-zero for received subvol
    pub stransid: u64,
    /// trans when received. non-zero for received subvol
    pub rtransid: u64,
    pub ctime: Timespec,
    pub otime: Timespec,
    pub stime: Timespec,
    pub rtime: Timespec,
    pub reserved: [u64; 8],
}
