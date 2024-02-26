use modular_bitfield::prelude::*;

pub mod ai;
pub mod mi;
pub mod pi;
pub mod si;
pub mod sp;
pub mod usb;
pub mod vi;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct DramAddr {
    addr: B24,
    #[skip]
    __: B8,
}
