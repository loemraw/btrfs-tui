use crate::structs::UUID_SIZE;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct DevItem {
    /// the internal btrfs device id
    pub devid: u64,
    /// size of the device
    pub total_bytes: u64,
    /// bytes used
    pub bytes_used: u64,
    /// optimal io alignment for this device
    pub io_align: u32,
    /// optimal io width for this device
    pub io_width: u32,
    /// minimal io size for this device
    pub sector_size: u32,
    /// type and info about this device
    pub ty: u64,
    /// expected generation for this device
    pub generation: u64,
    /// starting byte of this partition on the device, to allow for stripe alignment in the future
    pub start_offset: u64,
    /// grouping information for allocation decisions
    pub dev_group: u32,
    /// seek speed 0-100 where 100 is fastest
    pub seek_speed: u8,
    /// bandwidth 0-100 where 100 is fastest
    pub bandwidth: u8,
    /// btrfs generated uuid for this device
    pub uuid: [u8; UUID_SIZE],
    /// uuid of FS who owns this device
    pub fsid: [u8; UUID_SIZE],
}
