use crate::structs::Key;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DirItem {
    pub location: Key,
    pub transid: u64,
    pub data_len: u16,
    pub name_len: u16,
    pub ty: u8,
}
