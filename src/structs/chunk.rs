use crate::structs::Stripe;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Chunk {
    /// size of this chunk in bytes
    pub length: u64,
    /// objectid of the root referencing this chunk
    pub owner: u64,
    pub stripe_len: u64,
    pub ty: u64,
    /// optimal io alignment for this chunk
    pub io_align: u32,
    /// optimal io width for this chunk
    pub io_width: u32,
    /// minimal io size for this chunk
    pub sector_size: u32,
    /// 2^16 stripes is quite a lot, a second limit is the size of a single item in the btree
    pub num_stripes: u16,
    /// sub stripes only matter for raid10
    pub sub_stripes: u16,
    pub stripe: Stripe,
    // additional stripes go here
}
