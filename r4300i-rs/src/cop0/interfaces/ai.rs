use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Ctrl {
    dma: bool,
    #[skip]
    __: B31,
}

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

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct DacRate {
    rate: B14,
    #[skip]
    __: B18,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct BitRate {
    rate: B4,
    #[skip]
    __: B28,
}

#[derive(Debug)]
pub struct Ai {
    control: Ctrl,
    status: Status,
    dac_rate: DacRate,
    bit_rate: BitRate,
}

impl Ai {
    pub fn new() -> Self {
        Self {
            control: Ctrl::new(),
            status: Status::new(),
            dac_rate: DacRate::new(),
            bit_rate: BitRate::new(),
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04500008..=0x0450000B => {
                // control is write-only
                0
            }

            0x0450000C..=0x0450000F => retrieve_byte(
                self.status
                    .with_full_2_electric_boogaloo(self.status.full())
                    .into(),
                address,
            ),

            0x04500010..=0x04500013 => {
                // dacrate is write-only
                0
            }

            0x04500014..=0x04500017 => {
                // bitrate is write-only
                0
            }

            _ => {
                eprintln!("unimplemented AI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04500008..=0x0450000B => {
                self.control = merge_byte(self.control.into(), address, val).into()
            }

            0x0450000C..=0x0450000F => {
                //self.status = merge_byte(self.status.into(), address, val).into()
                // clear audio interrupt
            }

            0x04500010..=0x04500013 => {
                self.dac_rate = merge_byte(self.dac_rate.into(), address, val).into()
            }

            0x04500014..=0x04500017 => {
                self.bit_rate = merge_byte(self.bit_rate.into(), address, val).into()
            }

            _ => {
                eprintln!("unimplemented AI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}
