#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Timespec {
    pub sec: u64,
    pub nsec: u32,
}
