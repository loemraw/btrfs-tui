#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InodeRef {
    pub index: u64,
    pub name_len: u16,
}
