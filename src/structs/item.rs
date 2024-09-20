use crate::structs::Key;

#[repr(C, packed)]
#[derive(Copy, Clone)]
/// A `BtrfsLeaf` is full of `BtrfsItem`s. `offset` and `size` (relative to start of data area)
/// tell us where to find the item in the leaf.
pub struct Item {
    pub key: Key,
    pub offset: u32,
    pub size: u32,
}
