use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Status {
    dma_busy: bool,
    io_read_busy: bool,
    #[skip]
    __: B1,
    dma_error: bool,
    #[skip]
    __: B8,
    interrupt: bool,
    #[skip]
    __: B19,
}

#[derive(Debug)]
pub struct Si {
    status: Status,
}

impl Si {
    pub fn new() -> Self {
        Self {
            status: Status::new(),
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04800018..=0x0480001B => retrieve_byte(self.status.into(), address),
            _ => {
                eprintln!("unimplemented SI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04800018..=0x0480001B => {
                //self.status = merge_byte(self.status.into(), address, val).into()
                self.status.set_interrupt(false);
            }
            _ => {
                eprintln!("unimplemented SI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}
