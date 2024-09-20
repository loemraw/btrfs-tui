use crate::structs::Header;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Node {
    pub header: Header,
    // `BtrfsKeyPtr`s begin here
}
