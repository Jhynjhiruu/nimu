use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct ClockSel {
    clock: bool,
    #[skip]
    __: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SecMode {
    sec: bool,
    #[skip]
    __: B31,
}

#[derive(Debug)]
pub struct Usb {
    base_address: word,

    clock_sel: ClockSel,
    sec_mode: SecMode,

    sram: [u8; Self::SRAM_SIZE],
}

impl Usb {
    const SRAM_SIZE: usize = 0x200;

    const SRAM_START: word = 0x80000;
    const SRAM_END: word = Self::SRAM_START + Self::SRAM_SIZE as word;

    pub fn new(base_address: word) -> Self {
        Self {
            base_address,
            clock_sel: ClockSel::new(),
            sec_mode: SecMode::new(),
            sram: [0; Self::SRAM_SIZE],
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        let int_address = address - self.base_address;
        match int_address {
            0x1B => 0x20,

            0x0000..=0x0100 => {
                println!("ignored USB reg read @ {address:08X}");
                0
            }

            0x40000..=0x40003 => retrieve_byte(self.clock_sel.into(), address),

            0x40010..=0x40013 => retrieve_byte(self.sec_mode.into(), address),

            Self::SRAM_START..=Self::SRAM_END => {
                self.sram[(int_address - Self::SRAM_START) as usize]
            }

            _ => {
                eprintln!("unimplemented USB read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        let int_address = address - self.base_address;
        match int_address {
            0x0000..=0x0100 => {
                println!("ignored USB reg write @ {address:08X}: {val:02X}");
            }

            0x40000..=0x40003 => {
                self.clock_sel = merge_byte(self.clock_sel.into(), address, val).into()
            }

            0x40010..=0x40013 => {
                self.sec_mode = merge_byte(self.sec_mode.into(), address, val).into()
            }

            Self::SRAM_START..=Self::SRAM_END => {
                self.sram[(int_address - Self::SRAM_START) as usize] = val
            }

            _ => {
                eprintln!("unimplemented USB write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }

    pub fn step(&self) {
        
    }
}
