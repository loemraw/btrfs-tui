use crate::structs::UUID_SIZE;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Stripe {
    pub devid: u64,
    pub offset: u64,
    pub dev_uuid: [u8; UUID_SIZE],
}
