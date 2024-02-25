use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Status {
    full: bool,
    #[skip]
    __: B29,
    busy: bool,
    full_2_electric_boogaloo: bool,
}

#[derive(Debug)]
pub struct Ai {
    status: Status,
}

impl Ai {
    pub fn new() -> Self {
        Self {
            status: Status::new(),
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x0450000C..=0x0450000F => retrieve_byte(
                self.status
                    .with_full_2_electric_boogaloo(self.status.full())
                    .into(),
                address,
            ),
            _ => {
                eprintln!("unimplemented AI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x0450000C..=0x0450000F => {
                //self.status = merge_byte(self.status.into(), address, val).into()
                // clear audio interrupt
            }
            _ => {
                eprintln!("unimplemented AI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}
