use crate::types::*;

use modular_bitfield::prelude::*;

#[derive(Debug)]
pub struct ControllerConfig {
    crsto: [word; 2],
    crm: [word; 4],
}

impl ControllerConfig {
    pub fn new() -> Self {
        Self {
            crsto: [0; 2],
            crm: [0; 4],
        }
    }

    pub fn read_byte(&mut self, address: word) -> byte {
        match address {
            0x00..=0x03 => retrieve_byte(self.crsto[0], address),
            0x04..=0x07 => retrieve_byte(self.crsto[1], address),
            0x08..=0x0B => retrieve_byte(self.crm[0], address),
            0x0C..=0x0F => retrieve_byte(self.crm[1], address),
            0x10..=0x13 => retrieve_byte(self.crm[2], address),
            0x14..=0x17 => retrieve_byte(self.crm[3], address),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: word, val: byte) {
        match address {
            0x00..=0x03 => self.crsto[0] = merge_byte(self.crsto[0], address, val),
            0x04..=0x07 => self.crsto[1] = merge_byte(self.crsto[1], address, val),
            0x08..=0x0B => self.crm[0] = merge_byte(self.crm[0], address, val),
            0x0C..=0x0F => self.crm[1] = merge_byte(self.crm[1], address, val),
            0x10..=0x13 => self.crm[2] = merge_byte(self.crm[2], address, val),
            0x14..=0x17 => self.crm[3] = merge_byte(self.crm[3], address, val),
            _ => unreachable!(),
        }
    }
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Ctrl {
    pub porst: bool,
    pub unlock: bool,
    #[skip]
    __: B2,
    pub vppsel: B4,
    pub vrange: bool,
    pub data: bool,
    pub pe: bool,
    pub reset: bool,
    pub bias: B2,
    pub tecc: B2,
    pub mrcl: B2,
    pub comp: bool,
    #[skip]
    __: B1,
    pub store: bool,
    pub recall: bool,
    pub nv_match: bool,
    pub rcready: bool,
    pub cmd: B3,
    #[skip]
    __: B1,
    pub keep: bool,
    pub pass: bool,
    pub ready: bool,
    pub bypass: bool,
}

#[derive(Debug)]
pub struct Virage01 {
    config: ControllerConfig,
    data: [byte; Self::VIRAGE01_SIZE],
    sram: [byte; Self::INTERNAL_SRAM_SIZE],
    ctrl: Ctrl,
    nms: Ctrl,
    cp: Ctrl,

    command: byte,
}

impl Virage01 {
    const VIRAGE01_SIZE: usize = 0x40;
    const INTERNAL_SRAM_SIZE: usize = 0x40;

    pub fn new(mut data: Vec<byte>) -> Self {
        data.resize(Self::VIRAGE01_SIZE, 0);

        let data = data.try_into().unwrap();
        Self {
            config: ControllerConfig::new(),
            data,
            sram: data,
            ctrl: Ctrl::new().with_rcready(true).with_ready(true),
            nms: Ctrl::new().with_rcready(true).with_ready(true),
            cp: Ctrl::new().with_rcready(true).with_ready(true),

            command: 0,
        }
    }

    pub fn retrieve(&self) -> Vec<byte> {
        self.data.into()
    }

    pub fn get_ctrl(&mut self) -> word {
        if self.command == 2 {
            self.command = 0;
            self.data = self.sram;
            self.ctrl.with_ready(false).into()
        } else if self.command == 3 {
            self.command = 0;
            self.sram = self.data;
            self.ctrl.with_ready(true).into()
        } else {
            self.ctrl.with_ready(true).with_pass(true).into()
        }
    }

    pub fn set_ctrl(&mut self, val: word) {
        self.ctrl = val.into();
        self.command = self.ctrl.cmd();
    }

    pub fn get_nms(&self) -> word {
        self.nms.into()
    }

    pub fn set_nms(&mut self, val: word) {
        self.nms = val.into();
        self.command = self.nms.cmd();
    }

    pub fn get_cp(&self) -> word {
        self.cp.into()
    }

    pub fn set_cp(&mut self, val: word) {
        self.cp = val.into();
        self.command = self.cp.cmd();
    }
}

#[derive(Debug)]
pub struct Virage2 {
    config: ControllerConfig,
    data: [byte; Self::VIRAGE2_SIZE],
    sram: [byte; Self::INTERNAL_SRAM_SIZE],
    ctrl: Ctrl,
    nms: Ctrl,
    cp: Ctrl,

    command: byte,
}

impl Virage2 {
    const VIRAGE2_SIZE: usize = 0x100;
    const INTERNAL_SRAM_SIZE: usize = 0x100;

    pub fn new(mut data: Vec<byte>) -> Self {
        data.resize(Self::VIRAGE2_SIZE, 0);

        let data = data.try_into().unwrap();
        Self {
            config: ControllerConfig::new(),
            data,
            sram: data,
            ctrl: Ctrl::new().with_rcready(true).with_ready(true),
            nms: Ctrl::new().with_rcready(true).with_ready(true),
            cp: Ctrl::new().with_rcready(true).with_ready(true),

            command: 0,
        }
    }

    pub fn retrieve(&self) -> Vec<byte> {
        self.data.into()
    }

    pub fn get_ctrl(&mut self, reset: bool) -> word {
        if self.command == 2 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.data = self.sram;
            self.ctrl.with_ready(false).with_pass(true).into()
        } else if self.command == 3 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.sram = self.data;
            self.ctrl.with_ready(true).with_pass(true).into()
        } else {
            self.ctrl.with_ready(true).with_pass(true).into()
        }
    }

    pub fn set_ctrl(&mut self, val: word) {
        self.ctrl = val.into();
        self.command = self.ctrl.cmd();
    }

    pub fn get_nms(&mut self, reset: bool) -> word {
        if self.command == 2 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.data = self.sram;
            self.nms.with_ready(false).into()
        } else if self.command == 3 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.sram = self.data;
            self.nms.with_ready(true).into()
        } else {
            self.nms.with_ready(true).with_pass(true).into()
        }
    }

    pub fn set_nms(&mut self, val: word) {
        self.nms = val.into();
        self.command = self.nms.cmd();
    }

    pub fn get_cp(&mut self, reset: bool) -> word {
        if self.command == 2 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.data = self.sram;
            self.cp.with_ready(false).into()
        } else if self.command == 3 {
            if reset {
                self.command = 0;
                self.ctrl.set_cmd(0);
                self.nms.set_cmd(0);
                self.cp.set_cmd(0);
            }
            self.sram = self.data;
            self.cp.with_ready(true).into()
        } else {
            self.cp.with_ready(true).with_pass(true).into()
        }
    }

    pub fn set_cp(&mut self, val: word) {
        self.cp = val.into();
        self.command = self.cp.cmd();
    }
}

#[derive(Debug)]
pub struct Virage {
    bootrom: [byte; Self::BOOTROM_SIZE],
    bootram: [byte; Self::BOOTRAM_SIZE],

    sram: [byte; Self::SRAM_SIZE],

    bootrom_start: word,
    bootram_start: word,

    v0: Virage01,
    v1: Virage01,
    v2: Virage2,
}

impl Virage {
    const BOOTROM_SIZE: usize = 0x2000;
    const BOOTRAM_SIZE: usize = 0x10000;
    const SRAM_SIZE: usize = 0x8000;
    pub fn new(mut bootrom: Vec<byte>, v0: Vec<byte>, v1: Vec<byte>, v2: Vec<byte>) -> Self {
        bootrom.resize(Self::BOOTROM_SIZE, 0);

        let bootrom = bootrom.try_into().expect("bootrom already resized");

        Self {
            bootrom,
            bootram: [0; Self::BOOTRAM_SIZE],

            sram: [0; Self::SRAM_SIZE],

            bootrom_start: 0x1FC00000,
            bootram_start: 0x1FC20000,

            v0: Virage01::new(v0),
            v1: Virage01::new(v1),
            v2: Virage2::new(v2),
        }
    }

    pub fn retrieve_bootrom(&self) -> Vec<byte> {
        self.bootrom.into()
    }

    pub fn retrieve_v0(&self) -> Vec<byte> {
        self.v0.retrieve()
    }

    pub fn retrieve_v1(&self) -> Vec<byte> {
        self.v1.retrieve()
    }

    pub fn retrieve_v2(&self) -> Vec<byte> {
        self.v2.retrieve()
    }

    pub fn get_bootram(&self) -> &[u8] {
        &self.bootram
    }

    pub fn set_mapping(&mut self, mapping: bool) {
        let (rom, ram) = if mapping {
            (0x1FC00000, 0x1FC20000)
        } else {
            (0x1FC20000, 0x1FC00000)
        };
        self.bootrom_start = rom;
        self.bootram_start = ram;
    }

    fn _read_phys_addr(&mut self, address: word) -> byte {
        if (self.bootrom_start..self.bootrom_start + Self::BOOTROM_SIZE as word).contains(&address)
        {
            self.bootrom[(address - self.bootrom_start) as usize]
        } else if (self.bootram_start..self.bootram_start + Self::BOOTRAM_SIZE as word)
            .contains(&address)
        {
            self.bootram[(address - self.bootram_start) as usize]
        } else if (0x1FC40000..0x1FC48000).contains(&address) {
            self.sram[(address - 0x1FC40000) as usize]
        } else if (0x1FC80000..0x1FC80040).contains(&address) {
            self.v0.sram[(address - 0x1FC80000) as usize]
        } else if (0x1FC90000..0x1FC90040).contains(&address) {
            self.v1.sram[(address - 0x1FC90000) as usize]
        } else if (0x1FCA0000..0x1FCA0100).contains(&address) {
            self.v2.sram[(address - 0x1FCA0000) as usize]
        } else {
            eprintln!("unmapped virage read: {address:08X}");
            unimplemented!();
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x1FC88000..=0x1FC88017 => self.v0.config.read_byte(address - 0x1FC88000),

            0x1FC98000..=0x1FC98017 => self.v1.config.read_byte(address - 0x1FC98000),

            0x1FCA8000..=0x1FCA8017 => self.v2.config.read_byte(address - 0x1FCA8000),

            0x1FC8C000..=0x1FC8C003 => retrieve_byte(self.v0.get_ctrl(), address),
            0x1FC8D000..=0x1FC8D003 => retrieve_byte(self.v0.get_nms(), address),
            0x1FC8E000..=0x1FC8E003 => retrieve_byte(self.v0.get_cp(), address),
            0x1FC9C000..=0x1FC9C003 => retrieve_byte(self.v1.get_ctrl(), address),
            0x1FC9D000..=0x1FC9D003 => retrieve_byte(self.v1.get_nms(), address),
            0x1FC9E000..=0x1FC9E003 => retrieve_byte(self.v1.get_cp(), address),
            0x1FCAC000..=0x1FCAC003 => retrieve_byte(self.v2.get_ctrl(address & 3 == 3), address),
            0x1FCAD000..=0x1FCAD003 => retrieve_byte(self.v2.get_nms(address & 3 == 3), address),
            0x1FCAE000..=0x1FCAE003 => retrieve_byte(self.v2.get_cp(address & 3 == 3), address),
            _ => self._read_phys_addr(address),
        }
    }

    fn _write_phys_addr(&mut self, address: word, val: byte) {
        if (0x1FC40000..0x1FC48000).contains(&address) {
            self.sram[(address - 0x1FC40000) as usize] = val;
        } else if (self.bootram_start..self.bootram_start + Self::BOOTRAM_SIZE as word)
            .contains(&address)
        {
            self.bootram[(address - self.bootram_start) as usize] = val;
        } else if (0x1FC80000..0x1FC80040).contains(&address) {
            self.v0.sram[(address - 0x1FC80000) as usize] = val;
        } else if (0x1FC90000..0x1FC90040).contains(&address) {
            self.v1.sram[(address - 0x1FC90000) as usize] = val;
        } else if (0x1FCA0000..0x1FCA0100).contains(&address) {
            self.v2.sram[(address - 0x1FCA0000) as usize] = val;
        } else {
            eprintln!("unmapped virage write: {address:08X} {val:02X}");
            unimplemented!();
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x1FC88000..=0x1FC88017 => {
                self.v0.config.write_byte(address - 0x1FC88000, val);
            }

            0x1FC98000..=0x1FC98017 => {
                self.v1.config.write_byte(address - 0x1FC98000, val);
            }

            0x1FCA8000..=0x1FCA8017 => {
                self.v2.config.write_byte(address - 0x1FCA8000, val);
            }

            0x1FC8C000..=0x1FC8C003 => {
                let ctrl = self.v0.get_ctrl();
                self.v0.set_ctrl(merge_byte(ctrl, address, val))
            }
            0x1FC8E000..=0x1FC8E003 => self.v0.set_nms(merge_byte(self.v0.get_nms(), address, val)),
            0x1FC8D000..=0x1FC8D003 => self.v0.set_cp(merge_byte(self.v0.get_cp(), address, val)),
            0x1FC9C000..=0x1FC9C003 => {
                let ctrl = self.v1.get_ctrl();
                self.v1.set_ctrl(merge_byte(ctrl, address, val))
            }
            0x1FC9E000..=0x1FC9E003 => self.v1.set_nms(merge_byte(self.v1.get_nms(), address, val)),
            0x1FC9D000..=0x1FC9D003 => self.v1.set_cp(merge_byte(self.v1.get_cp(), address, val)),
            0x1FCAC000..=0x1FCAC003 => {
                let ctrl = self.v2.get_ctrl(false);
                self.v2.set_ctrl(merge_byte(ctrl, address, val))
            }
            0x1FCAD000..=0x1FCAD003 => {
                let nms = self.v2.get_nms(false);
                self.v2.set_nms(merge_byte(nms, address, val))
            }
            0x1FCAE000..=0x1FCAE003 => {
                let cp = self.v2.get_cp(false);
                self.v2.set_cp(merge_byte(cp, address, val))
            }
            _ => self._write_phys_addr(address, val),
        }
    }
}
