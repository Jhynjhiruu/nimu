use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Current {
    half_line: B10,
    #[skip] __: B22,
}

#[derive(Debug)]
pub struct Vi {
    current: Current,
}

impl Vi {
    pub fn new() -> Self {
        Self {current: Current::new()}
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04400010..=0x04400013 => retrieve_byte(self.current.into(), address),
    
            _ => {
                eprintln!("unimplemented VI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04400010..=0x04400013 => {
                self.current = merge_byte(self.current.into(), address, val).into()
            }
    
            _ => {
                eprintln!("unimplemented VI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}
