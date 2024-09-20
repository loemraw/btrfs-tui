use crate::structs::Key;

#[repr(C, packed)]
#[derive(Copy, Clone)]
/// All non-leaf blocks are nodes and they hold only keys are pointers to other blocks
pub struct KeyPtr {
    pub key: Key,
    pub blockptr: u64,
    pub generation: u64,
}
