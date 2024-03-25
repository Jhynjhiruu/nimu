use std::fs::write;

use r4300i_rs::R4300i;

#[derive(Debug)]
pub struct Nimu {
    cpu: R4300i,
}

//const PAYLOAD: &[u8] = include_bytes!(r"C:\Users\Jay\Documents\bbbs\payload");

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
        /*self.cpu.write::<u32>(0x80300000, 0x8006B940);
        self.cpu.write::<u32>(0x80300004, 0x00000000);
        self.cpu.write::<u32>(0x80300008, 0xA006C934);
        self.cpu.write::<u32>(0x8030000C, 0x24020030);
        self.cpu.write::<u32>(0x80300010, 0x3C01A460);
        self.cpu.write::<u32>(0x80300014, 0xAC220060);
        self.cpu.write::<u32>(0x80300018, 0x1000FFFF);
        self.cpu.write::<u32>(0x8030001C, 0x00000000);*/

        /*for (index, i) in PAYLOAD.iter().enumerate() {
            self.cpu.write(0x80300000 + index as u32, *i);
        }*/

        self.cpu.start();

        //self.cpu.start_logging();
        while !self.cpu.halted {
            self.cpu.step();

            if self.cpu.get_pc() as u32 == 0x9fc00000 && !self.cpu.get_mi_mapping() {
                write("kernel.bin", self.cpu.get_bootram()).unwrap();
                self.cpu.start_logging();
            }
            if self.cpu.get_pc() as u32 == 0x9fc00ea8 && !self.cpu.get_mi_mapping() {
                self.cpu.start_logging();
            }
            if self.cpu.get_pc() as u32 == 0x80002000 {
                write("ram.bin", self.cpu.get_ram()).unwrap();
            }
            /*if self.cpu.get_pc() as u32 == 0x9FC01F30 {
                self.cpu.start_logging();
            }*/
            if self.cpu.get_pc() as u32 == 0x9fc407c8 {
                println!(
                    "sk hash from v2: {:08X}{:08X}{:08X}{:08X}{:08X}",
                    self.cpu.read::<u32>(0xBFCA0000).unwrap(),
                    self.cpu.read::<u32>(0xBFCA0004).unwrap(),
                    self.cpu.read::<u32>(0xBFCA0008).unwrap(),
                    self.cpu.read::<u32>(0xBFCA000C).unwrap(),
                    self.cpu.read::<u32>(0xBFCA0010).unwrap()
                );

                let sp = self.cpu.get_reg(29) as u32;

                println!(
                    "calculated: {:08X}{:08X}{:08X}{:08X}{:08X}",
                    self.cpu.read::<u32>(sp + 0x90).unwrap(),
                    self.cpu.read::<u32>(sp + 0x94).unwrap(),
                    self.cpu.read::<u32>(sp + 0x98).unwrap(),
                    self.cpu.read::<u32>(sp + 0x9C).unwrap(),
                    self.cpu.read::<u32>(sp + 0xA0).unwrap()
                );
            }
            /*if self.cpu.get_pc() as u32 == 0x9fc032cc {
                println!(
                    "rsa_verify_signature -> {:08X}",
                    self.cpu.get_reg(31) as u32
                );
                self.cpu.stop_logging();
            }*/
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
                println!(
                    "ram at 0x80300000: {:08X} {:08X} {:08X} {:08X}",
                    self.cpu.read::<u32>(0x80300000).unwrap(),
                    self.cpu.read::<u32>(0x80300004).unwrap(),
                    self.cpu.read::<u32>(0x80300008).unwrap(),
                    self.cpu.read::<u32>(0x8030000C).unwrap()
                );

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
            if self.cpu.get_pc() as u32 == 0x9fc022a8 {
                //self.cpu.start_logging()
            }
            if self.cpu.get_pc() as u32 == 0x8000F654 {
                /*println!(
                    "sw <{:08X}>, 16({:08X})",
                    self.cpu.get_reg(0x10) as u32,
                    self.cpu.get_reg(0x1D) as u32
                );*/
            }
            if self.cpu.get_pc() as u32 == 0x800051b0 {
                //println!("osGetCount");
            }
            if self.cpu.get_pc() as u32 == 0x80005780 {
                println!("osStopThread");
            }
            if self.cpu.get_pc() as u32 == 0x800074CC {
                let thread_ptr = self.cpu.get_reg(26);
                println!("__osDispatchThread: {:016X}", thread_ptr);

                /*match thread_ptr as u32 {
                    0x8001AF48 | 0x8002B2A8 | 0x8006B818 => self.cpu.stop_logging(),
                    _ => self.cpu.start_logging(),
                }*/
            }
            if self.cpu.get_pc() as u32 == 0x800074FC {
                let k0 = self.cpu.get_reg(26) as u32;

                let mut dump = [0; 0x100];

                for (index, i) in dump.iter_mut().enumerate() {
                    *i = self.cpu.read::<u8>(k0 + index as u32).unwrap_or(0xEE);
                }

                write(format!("dump-{k0:08X}.bin"), dump).unwrap();
            }
            if self.cpu.get_pc() as u32 == 0x8000ad40 {
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

            if self.cpu.get_pc() as u32 == 0x80008924 {
                println!("__osBbIsBb: {}", self.cpu.read::<u32>(0x80000388).unwrap());
                println!("__osBbCardWaitEvent(): called osRecvMesg");
                // self.cpu.start_logging();
            }

            if self.cpu.get_pc() as u32 == 0x8000892C {
                println!("__osBbCardWaitEvent(): osRecvMesg returned");
            }

            if self.cpu.get_pc() as u32 == 0x8000672C {
                println!("t0 in exceptasm: {:08x}", self.cpu.get_reg(9) as u32);
            }

            if self.cpu.get_pc() as u32 == 0x80002050 {
                // patch sa1 to not check BBID
                // it's incredibly annoying that this is needed, but i can't
                // quite figure out interrupts properly without a rewrite
                // self.cpu.write::<u32>(0x80002100, 0x00000000);

                // patch sa1 to not wait for the interrupt reading sa2 to complete
                // self.cpu.write::<u32>(0x800083ac, 0x00000000);
            }
            /*if self.cpu.get_pc() as u32 == 0x80004f8c {
                if self.cpu.get_reg(3) as u32 == 0xA0006068 {
                    /*println!(
                        "ram at the place:\n{:08X} {:08X} {:08X} {:08X}\n{:08X} {:08X} {:08X} {:08X}",
                        self.cpu.read::<u32>(0xA006B934).unwrap(),
                        self.cpu.read::<u32>(0xA006B938).unwrap(),
                        self.cpu.read::<u32>(0xA006B93C).unwrap(),
                        self.cpu.read::<u32>(0xA006B940).unwrap(),
                        self.cpu.read::<u32>(0xA006B944).unwrap(),
                        self.cpu.read::<u32>(0xA006B948).unwrap(),
                        self.cpu.read::<u32>(0xA006B94C).unwrap(),
                        self.cpu.read::<u32>(0xA006B950).unwrap(),
                    );*/
                    self.cpu.start_logging();
                    //self.cpu.trigger_interrupt();
                    self.cpu.set_pc(0x80000000);
                }
            }*/
        }
        self.cpu.stop();
    }
}
