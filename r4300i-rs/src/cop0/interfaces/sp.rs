use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Status {
    bits: B15,
    #[skip]
    __: B17,
}

#[derive(Debug)]
pub struct Sp {
    status: Status,
}

impl Sp {
    pub fn new() -> Self {
        Self {
            status: Status::new(),
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04040010..=0x04040013 => retrieve_byte(self.status.into(), address),

            _ => {
                eprintln!("unimplemented SP read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04040010..=0x04040013 => {
                self.status = merge_byte(self.status.into(), address, val).into()
            }
            _ => {
                eprintln!("unimplemented SP write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }

    pub fn step(&self) {
        
    }
}
