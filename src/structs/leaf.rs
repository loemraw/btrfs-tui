use crate::structs::Header;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Leaf {
    pub header: Header,
    // `BtrfsItem`s begin here
}
