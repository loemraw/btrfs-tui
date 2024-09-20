#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Key {
    pub objectid: u64,
    pub ty: u8,
    pub offset: u64,
}
