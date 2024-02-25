use std::fmt::Debug;
use std::iter::IntoIterator;
use std::mem::size_of;

use crate::{is_ksegdm, k2_to_phys, kdm_to_phys, BOOTROM_BASE};
use crate::{types::*, ExceptionType};
use crate::{Exception, R4300i};

use num_traits::ops::bytes::{FromBytes, ToBytes};

mod interfaces;
pub mod registers;
mod tlb;
mod virage;

use interfaces::ai::Ai;
use interfaces::mi::Mi;
use interfaces::pi::{Dma, Pi};
use interfaces::si::Si;
use interfaces::sp::Sp;
use interfaces::usb::Usb;
use interfaces::vi::Vi;
use registers::*;
use tlb::TLBEntry;
use virage::Virage;

#[derive(Debug)]
pub struct State {
    registers: [word; Self::NUM_REGISTERS],
    coc: bool,
    tlb: [TLBEntry; Self::NUM_TLB_ENTRIES],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResetType {
    Cold,
    Warm,
}

impl ResetType {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Cold => false,
            Self::Warm => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    Index,
    Random,
    EntryLo0,
    EntryLo1,
    Context,
    PageMask,
    Wired,
    R7,
    BadVAddr,
    Count,
    EntryHi,
    Compare,
    Status,
    Cause,
    Epc,
    PrId,
    Config,
    LLAddr,
    WatchLo,
    WatchHi,
    XContext,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    CacheErr,
    TagLo,
    TagHi,
    ErrorEpc,
    R31,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Index,
            1 => Self::Random,
            2 => Self::EntryLo0,
            3 => Self::EntryLo1,
            4 => Self::Context,
            5 => Self::PageMask,
            6 => Self::Wired,
            7 => Self::R7,
            8 => Self::BadVAddr,
            9 => Self::Count,
            10 => Self::EntryHi,
            11 => Self::Compare,
            12 => Self::Status,
            13 => Self::Cause,
            14 => Self::Epc,
            15 => Self::PrId,
            16 => Self::Config,
            17 => Self::LLAddr,
            18 => Self::WatchLo,
            19 => Self::WatchHi,
            20 => Self::XContext,
            21 => Self::R21,
            22 => Self::R22,
            23 => Self::R23,
            24 => Self::R24,
            25 => Self::R25,
            26 => Self::R26,
            27 => Self::CacheErr,
            28 => Self::TagLo,
            29 => Self::TagHi,
            30 => Self::ErrorEpc,
            31 => Self::R31,
            _ => unreachable!(),
        }
    }
}

impl State {
    /// Registers:
    ///  0: Index
    ///  1: Random
    ///  2: EntryLo0
    ///  3: EntryLo1
    ///  4: Context
    ///  5: PageMask
    ///  6: Wired
    ///  7: <reserved>
    ///  8: BadVAddr
    ///  9: Count
    /// 10: EntryHi
    /// 11: Compare
    /// 12: Status
    /// 13: Cause
    /// 14: EPC
    /// 15: PRId
    /// 16: Config
    /// 17: LLAddr
    /// 18: WatchLo
    /// 19: WatchHi
    /// 20: XContext
    /// 21: <reserved>
    /// 22: <reserved>
    /// 23: <reserved>
    /// 24: <reserved>
    /// 25: <reserved>
    /// 26: <reserved>
    /// 27: CacheErr
    /// 28: TagLo
    /// 29: TagHi
    /// 30: ErrorEPC
    /// 31: <reserved>
    const NUM_REGISTERS: usize = 32;

    const NUM_TLB_ENTRIES: usize = 32;

    pub fn new(reset_type: ResetType) -> Self {
        let random = Random::new().with_random(31);

        let status = Status::new()
            .with_ts(false)
            .with_rp(false)
            .with_erl(true)
            .with_bev(true)
            .with_sr(reset_type.as_bool());

        let pr_id = PrId::new().with_imp(0xE7).with_rev(0xA5);

        let config = Config::new().with_ec(7).with_ep(0).with_be(true);

        let registers = [
            0,
            random.into(),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            status.into(),
            0,
            0,
            pr_id.into(),
            config.into(),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];

        let coc = false;

        let tlb = std::array::from_fn(|_| TLBEntry::new());

        Self {
            registers,
            coc,
            tlb,
        }
    }

    pub fn get_reg_raw(&self, reg: Register) -> word {
        self.registers[reg as usize]
    }

    pub fn set_reg_raw(&mut self, reg: Register, val: word) {
        if matches!(reg, Register::Wired) {
            self.registers[Register::Random as usize] = 31;
        }

        self.registers[reg as usize] = val;
    }

    pub fn get_reg<T>(&self, reg: Register) -> T
    where
        T: Cop0Register,
    {
        T::from_reg(self.get_reg_raw(reg), reg).expect("illegal register read")
    }

    pub fn set_reg<T>(&mut self, reg: Register, val: T)
    where
        T: Cop0Register,
    {
        self.registers[reg as usize] = val.as_reg(reg).expect("illegal register write");
    }

    pub fn get_coc(&self) -> bool {
        self.coc
    }

    pub fn set_coc(&mut self, val: bool) {
        self.coc = val;
    }

    pub fn get_tlb_entry(&self, index: usize) -> TLBEntry {
        self.tlb[index]
    }

    pub fn set_tlb_entry(&mut self, index: usize, entry: TLBEntry) {
        self.tlb[index] = entry;
    }

    pub fn read_tlb_entry_regs(&mut self, index: usize) {
        let tlb = self.tlb[index];
        self.set_reg(Register::PageMask, tlb.page_mask());
        self.set_reg(Register::EntryHi, tlb.entry_hi());

        self.set_reg(
            Register::EntryLo0,
            tlb.entry_lo_0().with_g(tlb.entry_hi().g()),
        );
        self.set_reg(
            Register::EntryLo1,
            tlb.entry_lo_1().with_g(tlb.entry_hi().g()),
        );
    }

    pub fn write_tlb_entry_regs(&mut self, index: usize) {
        let entry_lo_0: EntryLo = self.get_reg(Register::EntryLo0);
        let entry_lo_1: EntryLo = self.get_reg(Register::EntryLo1);

        let entry_hi: EntryHi = self.get_reg(Register::EntryHi);
        let page_mask: PageMask = self.get_reg(Register::PageMask);

        self.tlb[index] = TLBEntry::new()
            .with_entry_lo_0(entry_lo_0)
            .with_entry_lo_1(entry_lo_1)
            .with_entry_hi(entry_hi.with_g(entry_lo_0.g() && entry_lo_1.g()))
            .with_page_mask(page_mask);
    }

    pub fn page_size(mask: word) -> word {
        (mask + 1) << 12
    }
}

#[derive(Debug)]
pub struct Cop0 {
    pub state: State,

    ram: Box<[byte; Self::RAM_SIZE]>,

    virage: Virage,

    sp: Sp,
    mi: Mi,
    vi: Vi,
    ai: Ai,
    pi: Pi,
    si: Si,
    usb0: Usb,
    usb1: Usb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TLBResult<T> {
    Ok(T),
    Shutdown,
    Exception(Exception),
}

impl Cop0 {
    const RAM_SIZE: usize = 0x00800000;

    pub fn new(
        reset_type: ResetType,
        bootrom: Vec<byte>,
        v0: Vec<byte>,
        v1: Vec<byte>,
        v2: Vec<byte>,
        nand: Vec<byte>,
        spare: Vec<byte>,
    ) -> Self {
        Self {
            state: State::new(reset_type),
            ram: Box::new([0; Self::RAM_SIZE]),
            virage: Virage::new(bootrom, v0, v1, v2),
            sp: Sp::new(),
            mi: Mi::new(),
            vi: Vi::new(),
            ai: Ai::new(),
            pi: Pi::new(nand, spare),
            si: Si::new(),
            usb0: Usb::new(0x04900000),
            usb1: Usb::new(0x04A00000),
        }
    }

    pub fn retrieve_bootrom(&self) -> Vec<byte> {
        self.virage.retrieve_bootrom()
    }

    pub fn retrieve_v0(&self) -> Vec<byte> {
        self.virage.retrieve_v0()
    }

    pub fn retrieve_v1(&self) -> Vec<byte> {
        self.virage.retrieve_v1()
    }

    pub fn retrieve_v2(&self) -> Vec<byte> {
        self.virage.retrieve_v2()
    }

    pub fn retrieve_nand(&self) -> Vec<byte> {
        self.pi.retrieve_nand()
    }

    pub fn retrieve_spare(&self) -> Vec<byte> {
        self.pi.retrieve_spare()
    }

    pub fn get_bootram(&self) -> &[u8] {
        self.virage.get_bootram()
    }

    pub fn get_mi_mapping(&self) -> bool {
        self.mi.get_sec_mode_map()
    }

    fn virt_to_phys(&mut self, address: word, write: bool) -> TLBResult<word> {
        if is_ksegdm(address) {
            TLBResult::Ok(kdm_to_phys(address))
        } else {
            let mut hits = 0;

            let mut p_addr = 0;

            for i in 0..State::NUM_TLB_ENTRIES {
                let entry = self.state.get_tlb_entry(i);

                let mask = entry.page_mask();
                let hi = entry.entry_hi();
                let lo0 = entry.entry_lo_0();
                let lo1 = entry.entry_lo_1();

                let mask = mask.mask() as word;

                let size = State::page_size(mask);

                let asid = (address >> 24) as byte;
                let address = address & 0x00FFFFFF;

                let vpn = address & ((!mask) << 13);
                let lo = if (address & size) > 0 { lo1 } else { lo0 };

                if (vpn >> 1) == hi.vpn() && (lo.g() || asid == hi.asid()) {
                    hits += 1;
                    if hits > 1 {
                        let mut status: Status = self.state.get_reg(Register::Status);
                        status.set_ts(true);
                        self.state.set_reg(Register::Status, status);

                        return TLBResult::Shutdown;
                    }

                    if !lo.v() {
                        self.state
                            .set_reg(Register::BadVAddr, BadVAddr::new().with_bad_vaddr(address));

                        return TLBResult::Exception(Exception::new_tlb(
                            if write {
                                ExceptionType::TLBMissWrite
                            } else {
                                ExceptionType::TLBMissRead
                            },
                            true,
                        ));
                    }

                    if !lo.d() && write {
                        self.state
                            .set_reg(Register::BadVAddr, BadVAddr::new().with_bad_vaddr(address));

                        return TLBResult::Exception(Exception::new(
                            ExceptionType::TLBModification,
                        ));
                    }

                    p_addr = (lo.pfn() as word).wrapping_mul(size) | (address ^ vpn);
                }
            }

            if hits == 0 {
                self.state
                    .set_reg(Register::BadVAddr, BadVAddr::new().with_bad_vaddr(address));

                let mut context: Context = self.state.get_reg(Register::Context);
                context.set_bad_vpn(address >> 13);
                self.state.set_reg(Register::Context, context);

                TLBResult::Exception(Exception::new(if write {
                    ExceptionType::TLBMissWrite
                } else {
                    ExceptionType::TLBMissRead
                }))
            } else {
                TLBResult::Ok(p_addr)
            }
        }
    }

    fn read_byte(&mut self, address: word) -> TLBResult<byte> {
        let address = match self.virt_to_phys(address, false) {
            TLBResult::Ok(a) => a,
            TLBResult::Shutdown => return TLBResult::Shutdown,
            TLBResult::Exception(e) => return TLBResult::Exception(e),
        };

        let watch_lo: WatchLo = self.state.get_reg(Register::WatchLo);

        if watch_lo.r() && (address & !7) == watch_lo.p_addr() {
            return TLBResult::Exception(Exception::new(ExceptionType::Watch));
        }

        TLBResult::Ok(self.read_phys_addr(address))
    }

    fn write_byte(&mut self, address: word, val: byte) -> TLBResult<()> {
        let address = match self.virt_to_phys(address, true) {
            TLBResult::Ok(a) => a,
            TLBResult::Shutdown => return TLBResult::Shutdown,
            TLBResult::Exception(e) => return TLBResult::Exception(e),
        };

        let watch_lo: WatchLo = self.state.get_reg(Register::WatchLo);

        if watch_lo.w() && (address & !7) == watch_lo.p_addr() {
            return TLBResult::Exception(Exception::new(ExceptionType::Watch));
        }

        self.write_phys_addr(address, val);

        TLBResult::Ok(())
    }

    pub fn read<T: FromBytes>(&mut self, address: word) -> TLBResult<T>
    where
        <T as FromBytes>::Bytes: Sized + TryFrom<Vec<u8>>,
        <<T as FromBytes>::Bytes as TryFrom<Vec<u8>>>::Error: Debug,
    {
        let size = size_of::<T>();

        if address % (size as word) != 0 {
            return TLBResult::Exception(Exception::new(ExceptionType::AddressErrorRead));
        }

        let mut bytes = vec![0u8; size];

        for (index, b) in bytes.iter_mut().enumerate() {
            *b = match self.read_byte(address.wrapping_add(index as word)) {
                TLBResult::Ok(b) => b,
                TLBResult::Shutdown => return TLBResult::Shutdown,
                TLBResult::Exception(e) => return TLBResult::Exception(e),
            };
        }

        let bytes = bytes.try_into().expect("should never fail");

        TLBResult::Ok(T::from_be_bytes(&bytes))
    }

    pub fn write<T: ToBytes>(&mut self, address: word, val: T) -> TLBResult<()>
    where
        <T as ToBytes>::Bytes: IntoIterator<Item = byte>,
    {
        let size = size_of::<T>();

        if address % (size as word) != 0 {
            return TLBResult::Exception(Exception::new(ExceptionType::AddressErrorWrite));
        }

        let bytes = val.to_be_bytes();

        for (index, b) in bytes.into_iter().enumerate() {
            match self.write_byte(address + index as word, b) {
                TLBResult::Ok(_) => {}
                TLBResult::Shutdown => return TLBResult::Shutdown,
                TLBResult::Exception(e) => return TLBResult::Exception(e),
            }
        }

        TLBResult::Ok(())
    }

    pub fn dma(&mut self) {
        let dma_queued = self.pi.dma_queued();

        if dma_queued {
            match self.pi.dma_params() {
                Dma::Read(dram_addr, pi_addr, len) => {
                    self.pi.bus_write(
                        pi_addr,
                        len,
                        &self.ram[dram_addr as usize..(dram_addr + len) as usize],
                    );
                }
                Dma::Write(dram_addr, pi_addr, len) => {
                    self.ram[dram_addr as usize..(dram_addr + len) as usize]
                        .copy_from_slice(self.pi.bus_read(pi_addr, len));
                }
                Dma::BufRead(dram_addr, pi_addr, len) => {
                    let len = len.min(0x400);
                    self.pi.buf_write(
                        pi_addr,
                        len,
                        &self.ram[dram_addr as usize..(dram_addr + len) as usize],
                    );
                }
                Dma::BufWrite(dram_addr, pi_addr, len) => {
                    let len = len.min(0x400);
                    println!("dram_addr: {dram_addr:08X}, pi_addr: {pi_addr:08X}, len: {len:08X}");
                    self.ram[dram_addr as usize..(dram_addr + len) as usize]
                        .copy_from_slice(self.pi.buf_read(pi_addr, len));
                }
            }

            self.pi.clear_dma();
        }
    }

    fn read_phys_addr(&mut self, address: word) -> byte {
        // should come back and tidy this up later
        // probably won't

        if (0x00000000..0x03F00000).contains(&address) {
            self.ram[address as usize]
        } else if (0x03F00000..0x04000000).contains(&address) {
            todo!()
        } else if (0x04000000..0x04100000).contains(&address) {
            self.sp.read_phys_addr(address)
        } else if (0x04100000..0x04200000).contains(&address) {
            // dp
            todo!()
        } else if (0x04200000..0x04300000).contains(&address) {
            // dp span
            todo!()
        } else if (0x04300000..0x04400000).contains(&address) {
            self.mi.read_phys_addr(address)
        } else if (0x04400000..0x04500000).contains(&address) {
            self.vi.read_phys_addr(address)
        } else if (0x04500000..0x04600000).contains(&address) {
            self.ai.read_phys_addr(address)
        } else if (0x04600000..0x04700000).contains(&address) {
            self.pi.read_phys_addr(address)
        } else if (0x04700000..0x04800000).contains(&address) {
            // ri
            // we are going to ignore ri for now
            0
        } else if (0x04800000..0x04900000).contains(&address) {
            self.si.read_phys_addr(address)
        } else if (0x04900000..0x04A00000).contains(&address) {
            self.usb0.read_phys_addr(address)
        } else if (0x04A00000..0x04B00000).contains(&address) {
            self.usb1.read_phys_addr(address)
        } else if (0x04B00000..0x05000000).contains(&address) {
            // open bus
            todo!()
        } else if (0x05000000..0x06000000).contains(&address) {
            // cartridge domain 2
            todo!()
        } else if (0x06000000..0x08000000).contains(&address) {
            // cartridge domain 1
            todo!()
        } else if (0x08000000..0x10000000).contains(&address) {
            // cartridge domain 2
            todo!()
        } else if (0x10000000..0x1FC00000).contains(&address) {
            // cartridge domain 1
            todo!()
        } else if (0x1FC00000..0x1FD00000).contains(&address) {
            self.virage.read_phys_addr(address)
        } else {
            println!("unmapped read: {:08X}", address);
            0
        }
    }

    fn write_phys_addr(&mut self, address: word, val: byte) {
        if (0x00000000..0x03F00000).contains(&address) {
            self.ram[address as usize] = val;
        } else if (0x03F00000..0x04000000).contains(&address) {
            todo!()
        } else if (0x04000000..0x04100000).contains(&address) {
            self.sp.write_phys_addr(address, val);
        } else if (0x04100000..0x04200000).contains(&address) {
            // dp
            todo!()
        } else if (0x04200000..0x04300000).contains(&address) {
            // dp span
            todo!()
        } else if (0x04300000..0x04400000).contains(&address) {
            self.mi.write_phys_addr(address, val);
            if self.mi.mapping_changed {
                self.mi.mapping_changed = false;
                self.virage.set_mapping(self.mi.get_sec_mode_map());
            }
        } else if (0x04400000..0x04500000).contains(&address) {
            self.vi.write_phys_addr(address, val);
        } else if (0x04500000..0x04600000).contains(&address) {
            self.ai.write_phys_addr(address, val);
        } else if (0x04600000..0x04700000).contains(&address) {
            self.pi.write_phys_addr(address, val);
        } else if (0x04700000..0x04800000).contains(&address) {
            // ri
            // we are going to ignore ri for now
        } else if (0x04800000..0x04900000).contains(&address) {
            self.si.write_phys_addr(address, val);
        } else if (0x04900000..0x04A00000).contains(&address) {
            self.usb0.write_phys_addr(address, val);
        } else if (0x04A00000..0x04B00000).contains(&address) {
            self.usb1.write_phys_addr(address, val);
        } else if (0x04B00000..0x05000000).contains(&address) {
            // open bus
            todo!()
        } else if (0x05000000..0x06000000).contains(&address) {
            // cartridge domain 2
            todo!()
        } else if (0x06000000..0x08000000).contains(&address) {
            // cartridge domain 1
            todo!()
        } else if (0x08000000..0x10000000).contains(&address) {
            // cartridge domain 2
            todo!()
        } else if (0x10000000..0x1FC00000).contains(&address) {
            // cartridge domain 1
            todo!()
        } else if (0x1FC00000..0x1FD00000).contains(&address) {
            self.virage.write_phys_addr(address, val);
        } else {
            println!("unmapped write: {:08X} {:02X}", address, val);
        }
    }
}
