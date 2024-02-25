use std::fs::write;

use r4300i_rs::R4300i;

#[derive(Debug)]
pub struct Nimu {
    cpu: R4300i,
}

impl Nimu {
    pub fn new(
        bootrom: Vec<u8>,
        v0: Vec<u8>,
        v1: Vec<u8>,
        v2: Vec<u8>,
        nand: Vec<u8>,
        spare: Vec<u8>,
    ) -> Self {
        Self {
            cpu: R4300i::new(bootrom, v0, v1, v2, nand, spare),
        }
    }

    pub fn run(&mut self) {
        self.cpu.start();
        while !self.cpu.halted {
            self.cpu.step();
            if self.cpu.get_pc() as u32 == 0x9fc00000 && !self.cpu.get_mi_mapping() {
                write("kernel.bin", self.cpu.get_bootram()).unwrap();
                //self.cpu.start_logging();
            }
            /*if self.cpu.get_pc() as u32 == 0x9FC01F30 {
                self.cpu.start_logging();
            }*/
            if self.cpu.get_pc() as u32 == 0x9fc032cc {
                println!("rsa -> {:08X}", self.cpu.get_reg(31) as u32);
                self.cpu.stop_logging();
            }
            /*if self.cpu.get_pc() as u32 == 0x9fc03890 {
                println!("param: {:016X}", self.cpu.get_reg(4));
            }
            if self.cpu.get_pc() as u32 == 0x9fc01b60 {
                println!("link from: {:016X}", self.cpu.get_reg(4));
            }
            if self.cpu.get_pc() as u32 == 0x9fc01c1c {
                println!("good block: {:016X}", self.cpu.get_reg(2));
            }*/
            /*if self.cpu.get_pc() as u32 == 0x9fc037fc {
                let (key, iv) = (self.cpu.get_reg(4), self.cpu.get_reg(5));
                println!("{key:016X}, {iv:016X}");
                let mut key_buf = [0; 0x14];
                let mut iv_buf = [0; 0x14];
                for (index, (k, i)) in key_buf.iter_mut().zip(iv_buf.iter_mut()).enumerate() {
                    *k = self.cpu.read::<u8>(key as u32 + index as u32).unwrap_or(0);
                    *i = self.cpu.read::<u8>(iv as u32 + index as u32).unwrap_or(0);
                }

                println!("{key_buf:02X?}\n{iv_buf:02X?}");
            }*/
            if self.cpu.get_pc() as u32 == 0x9fc02458 {
                let sp = self.cpu.get_reg(29);
                let ptr = sp + 0x10;
                let addr = self.cpu.read::<u32>(ptr as _).unwrap_or(0);
                println!("load addr: {addr:08X}");
                let mut sa1_buf = [0; 0x1C000];
                for (index, p) in sa1_buf.iter_mut().enumerate() {
                    *p = self.cpu.read::<u8>(addr + index as u32).unwrap_or(0);
                }
                write("sysapp.bin", sa1_buf).unwrap();
            }
            /*if self.cpu.get_pc() as u32 == 0x9fc03b1c && self.cpu.get_reg(31) as u32 == 0x9FC02270 {
                let (known, calculated) = (self.cpu.get_reg(4), self.cpu.get_reg(5));
                println!("{known:016X}, {calculated:016X}");
                let mut known_buf = [0; 0x14];
                let mut calculated_buf = [0; 0x14];
                for (index, (k, c)) in known_buf
                    .iter_mut()
                    .zip(calculated_buf.iter_mut())
                    .enumerate()
                {
                    *k = self
                        .cpu
                        .read::<u8>(known as u32 + index as u32)
                        .unwrap_or(0);
                    *c = self
                        .cpu
                        .read::<u8>(calculated as u32 + index as u32)
                        .unwrap_or(0);
                }

                println!("{known_buf:02X?}\n{calculated_buf:02X?}");
            }*/
            if self.cpu.get_pc() as u32 == 0xBFC03F00 {
                self.cpu.halt();
            }
            if self.cpu.get_pc() as u32 == 0xBFC00A7C {
                let mut a0 = self.cpu.get_reg(4);

                loop {
                    let c = self.cpu.read::<u8>(a0 as u32).unwrap_or(0);
                    a0 += 1;
                    if c == 0 {
                        break;
                    }
                    print!("{}", c as char);
                }
            }
            if self.cpu.get_pc() as u32 == 0xBFC00380 {
                println!("ra: {:08X}", self.cpu.get_reg(31));
            }
            if self.cpu.get_pc() as u32 == 0x9fc03fe8 {
                //self.cpu.start_logging();
                println!("virage write {:016X}", self.cpu.get_reg(4));
            }
        }
        self.cpu.stop();
    }
}
