use crate::structs::Timespec;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InodeItem {
    /// nfs style generation number
    pub generation: u64,
    /// transid that last touched this inode
    pub transid: u64,
    pub size: u64,
    pub nbytes: u64,
    pub block_group: u64,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub rdev: u64,
    pub flags: u64,
    /// modification sequence number for NFS
    pub sequence: u64,
    pub reserved: [u64; 4],
    pub atime: Timespec,
    pub ctime: Timespec,
    pub mtime: Timespec,
    pub otime: Timespec,
}
