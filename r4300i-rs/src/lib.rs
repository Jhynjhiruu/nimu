use std::collections::VecDeque;
use std::fmt::Debug;

use num_traits::{FromBytes, ToBytes};

mod cop0;
mod instruction;
pub mod types;

use cop0::{Cop0, ResetType};
use instruction::execute::{DelaySlot, InstructionFunction};
use instruction::Instruction;
use types::*;

pub const BOOTROM_BASE: word = 0xBFC00000;
pub const RAM_BASE: word = 0x80000000;

pub const KUBASE: word = 0x00000000;
pub const KUSIZE: word = 0x80000000;
pub const K0BASE: word = KUBASE + KUSIZE;
pub const K0SIZE: word = 0x20000000;
pub const K1BASE: word = K0BASE + K0SIZE;
pub const K1SIZE: word = 0x20000000;
pub const K2BASE: word = K1BASE + K1SIZE;
pub const K2SIZE: word = 0x20000000;

pub const KPTE_SHDUBASE: word = K2BASE + K2SIZE;

pub fn k0_to_k1(address: word) -> word {
    address | K1BASE
}

pub fn k1_to_k0(address: word) -> word {
    address & (K1BASE - 1)
}

pub fn k0_to_phys(address: word) -> word {
    address & (K0SIZE - 1)
}

pub fn k1_to_phys(address: word) -> word {
    address & (K1SIZE - 1)
}

pub fn k2_to_phys(address: word) -> word {
    address & (K2SIZE - 1)
}

pub fn kdm_to_phys(address: word) -> word {
    if is_kseg0(address) {
        k0_to_phys(address)
    } else if is_kseg1(address) {
        k1_to_phys(address)
    } else if is_kseg2(address) {
        k2_to_phys(address)
    } else {
        address
    }
}

pub fn phys_to_k0(address: word) -> word {
    address | K0BASE
}

pub fn phys_to_k1(address: word) -> word {
    address | K1BASE
}

pub fn is_kseg0(address: word) -> bool {
    (K0BASE..K1BASE).contains(&address)
}

pub fn is_kseg1(address: word) -> bool {
    (K1BASE..K2BASE).contains(&address)
}

pub fn is_ksegdm(address: word) -> bool {
    is_kseg0(address) || is_kseg1(address)
}

pub fn is_kseg2(address: word) -> bool {
    (K2BASE..KPTE_SHDUBASE).contains(&address)
}

pub fn is_kpteseg(address: word) -> bool {
    address >= KPTE_SHDUBASE
}

pub fn is_kuseg(address: word) -> bool {
    address < K0BASE
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::At,
            2 => Self::V0,
            3 => Self::V1,
            4 => Self::A0,
            5 => Self::A1,
            6 => Self::A2,
            7 => Self::A3,
            8 => Self::T0,
            9 => Self::T1,
            10 => Self::T2,
            11 => Self::T3,
            12 => Self::T4,
            13 => Self::T5,
            14 => Self::T6,
            15 => Self::T7,
            16 => Self::S0,
            17 => Self::S1,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::T8,
            25 => Self::T9,
            26 => Self::K0,
            27 => Self::K1,
            28 => Self::Gp,
            29 => Self::Sp,
            30 => Self::Fp,
            31 => Self::Ra,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FpRegister {
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FpControl {
    Version,
    Control,
}

impl From<u8> for FpControl {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Version,
            31 => Self::Control,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pc: dword,
    hi: dword,
    lo: dword,
    llbit: bool,

    registers: [dword; Self::NUM_REGISTERS],
    fp_registers: [f64; Self::NUM_FP_REGISTERS],

    fp_control_registers: [word; 2],
}

impl State {
    const NUM_REGISTERS: usize = 32;
    const NUM_FP_REGISTERS: usize = 32;

    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_pc(&self) -> dword {
        self.pc
    }

    pub fn set_pc(&mut self, val: dword) {
        self.pc = val;
    }

    pub fn get_hi(&self) -> dword {
        self.hi
    }

    pub fn set_hi(&mut self, val: dword) {
        self.hi = val;
    }

    pub fn get_lo(&self) -> dword {
        self.lo
    }

    pub fn set_lo(&mut self, val: dword) {
        self.lo = val;
    }

    pub fn get_llbit(&self) -> bool {
        self.llbit
    }

    pub fn set_llbit(&mut self, val: bool) {
        self.llbit = val;
    }

    pub fn get_reg(&self, reg: Register) -> dword {
        if matches!(reg, Register::Zero | Register::Gp) {
            0
        } else {
            self.registers[reg as usize]
        }
    }

    pub fn set_reg(&mut self, reg: Register, val: dword) {
        if !matches!(reg, Register::Zero | Register::Gp) {
            self.registers[reg as usize] = val;
        }
    }

    pub fn get_fp_reg(&self, reg: FpRegister) -> f64 {
        self.fp_registers[reg as usize]
    }

    pub fn set_fp_reg(&mut self, reg: FpRegister, val: f64) {
        self.fp_registers[reg as usize] = val;
    }

    pub fn get_fp_control_reg(&self, reg: FpControl) -> word {
        self.fp_control_registers[reg as usize]
    }

    pub fn set_fp_control_reg(&mut self, reg: FpControl, val: word) {
        self.fp_control_registers[reg as usize] = val;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExceptionType {
    Interrupt,
    TLBModification,
    TLBMissRead,
    TLBMissWrite,
    AddressErrorRead,
    AddressErrorWrite,
    BusErrorF,
    BusErrorLS,
    Syscall,
    Breakpoint,
    ReservedInstruction,
    CoprocessorUnusable,
    ArithmeticOverflow,
    Trap,
    E14,
    FloatingPoint,
    E16,
    E17,
    E18,
    E19,
    E20,
    E21,
    E22,
    Watch,
    E24,
    E25,
    E26,
    E27,
    E28,
    E29,
    E30,
    E31,
    ColdReset,
    SoftReset,
    NMI,
    None,
}

impl Default for ExceptionType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpuExceptionBit {
    InexactOperation,
    Underflow,
    Overflow,
    DivisionByZero,
    InvalidOperation,
    UnimplementedOperation,
}

impl Default for FpuExceptionBit {
    fn default() -> Self {
        Self::InexactOperation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Exception {
    exception: ExceptionType,
    tlb_invalid: bool,
    fpu_exception_bit: FpuExceptionBit,
}

impl Exception {
    pub fn new(exception: ExceptionType) -> Self {
        Self {
            exception,
            tlb_invalid: false,
            fpu_exception_bit: Default::default(),
        }
    }

    pub fn new_tlb(exception: ExceptionType, tlb_invalid: bool) -> Self {
        Self {
            exception,
            tlb_invalid,
            fpu_exception_bit: Default::default(),
        }
    }

    pub fn new_fp(
        exception: ExceptionType,
        tlb_invalid: bool,
        fpu_exception_bit: FpuExceptionBit,
    ) -> Self {
        Self {
            exception,
            tlb_invalid,
            fpu_exception_bit,
        }
    }

    pub fn priority(&self) -> usize {
        match self.exception {
            ExceptionType::ColdReset => 19,
            ExceptionType::SoftReset => 18,
            ExceptionType::NMI => 17,
            ExceptionType::AddressErrorWrite => 16,
            ExceptionType::TLBMissWrite => 15,
            ExceptionType::BusErrorF => 14,
            ExceptionType::Syscall => 13,
            ExceptionType::Breakpoint => 12,
            ExceptionType::CoprocessorUnusable => 11,
            ExceptionType::ReservedInstruction => 10,
            ExceptionType::Trap => 9,
            ExceptionType::ArithmeticOverflow => 8,
            ExceptionType::FloatingPoint => 7,
            ExceptionType::AddressErrorRead => 6,
            ExceptionType::TLBMissRead => 5,
            ExceptionType::TLBModification => 4,
            ExceptionType::Watch => 3,
            ExceptionType::BusErrorLS => 2,
            ExceptionType::Interrupt => 1,
            ExceptionType::None => 0,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureTrapType {
    Button,
    Emulation,
    Fatal,
    Timer,
    App,
}

#[derive(Debug)]
pub struct R4300i {
    state: State,
    cop0: Cop0,

    coc0: bool,
    coc1: bool,

    delay_slot: VecDeque<DelaySlot>,
    is_branch_likely: bool,

    did_cold_reset: bool,
    did_soft_reset: bool,
    did_nmi: bool,

    running: bool,
    pub halted: bool,

    logging: bool,

    prev_instruction: Option<Instruction>,
    cur_instruction: Option<Instruction>,

    exception: Exception,

    video_timer: word,
}

impl R4300i {
    const RESET_PC: dword = BOOTROM_BASE as dword;

    const EXCEPTION_PC: dword = 0x80000000;
    const EXCEPTION_PC_BEV: dword = 0xBFC00200;

    const TLB_MISS_ADD: dword = 0x0000;
    const XTLB_MISS_ADD: dword = 0x0080;
    const OTHER_ADD: dword = 0x0180;

    const SK_ENTER: dword = 0x9FC00000;

    pub fn new(
        bootrom: Vec<byte>,
        v0: Vec<byte>,
        v1: Vec<byte>,
        v2: Vec<byte>,
        nand: Vec<byte>,
        spare: Vec<byte>,
    ) -> Self {
        let did_cold_reset = false;
        let did_soft_reset = false;
        let did_nmi = false;

        let reset_type = if did_soft_reset | did_nmi {
            ResetType::Warm
        } else {
            ResetType::Cold
        };

        let mut state = State::new();

        state.set_pc(Self::RESET_PC);

        Self {
            state,
            cop0: Cop0::new(reset_type, bootrom, v0, v1, v2, nand, spare),
            coc0: false,
            coc1: false,
            delay_slot: VecDeque::new(),
            is_branch_likely: false,
            did_cold_reset,
            did_soft_reset,
            did_nmi,
            running: false,
            halted: false,
            logging: false,
            prev_instruction: None,
            cur_instruction: None,
            exception: Exception::default(),
            video_timer: 0,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn halt(&mut self) {
        self.running = false;
        self.halted = true;
    }

    pub fn start_logging(&mut self) {
        self.logging = true;
    }

    pub fn stop_logging(&mut self) {
        self.logging = false;
    }

    pub fn trigger_interrupt(&mut self) {
        self.cop0.trigger_md_intr();
    }

    pub fn step(&mut self) {
        self.did_cold_reset = false;
        self.did_soft_reset = false;
        self.did_nmi = false;

        if self.running && !self.halted {
            let coc0buf = self.cop0.state.get_coc();
            let coc1buf = false; // TODO cop1

            self.fetch_instruction();
            self.execute_instruction();

            self.coc0 = coc0buf;
            self.coc1 = coc1buf;

            self.handle_exception();

            if self.cop0.should_raise_interrupt() {
                println!("Interrupt!");
                self.throw_exception(Exception::new(ExceptionType::Interrupt));
            }

            self.cop0.step();
        }

        if self.logging {
            //println!("{:016X?}", self.state.registers);
            /*println!(
                "{:08X}",
                self.cop0
                    .state
                    .get_reg::<crate::cop0::registers::Epc>(cop0::Register::Epc)
                    .epc()
            )*/
        }
    }

    pub fn read<T>(&mut self, address: word) -> Option<T>
    where
        T: FromBytes,
        <T as FromBytes>::Bytes: TryFrom<Vec<u8>>,
        <<T as FromBytes>::Bytes as TryFrom<Vec<u8>>>::Error: Debug,
    {
        match self.cop0.read(address) {
            cop0::TLBResult::Ok(d) => Some(d),
            cop0::TLBResult::Shutdown => {
                self.halt();
                None
            }
            cop0::TLBResult::Exception(e) => {
                println!("exception at {:016X}", self.get_pc());
                self.throw_exception(e);
                None
            }
            cop0::TLBResult::SecureTrap(t) => {
                println!("secure trap at {:016X}", self.get_pc());
                self.secure_trap(t);
                None
            }
        }
    }

    pub fn write<T>(&mut self, address: word, val: T)
    where
        T: ToBytes,
        <T as ToBytes>::Bytes: IntoIterator<Item = byte>,
    {
        match self.cop0.write(address, val) {
            cop0::TLBResult::Ok(_) => {}
            cop0::TLBResult::Shutdown => self.halt(),
            cop0::TLBResult::Exception(e) => {
                println!("exception at {:016X}", self.get_pc());
                self.halt();
                self.throw_exception(e)
            }
            _ => unreachable!(),
        }
    }

    fn fetch_instruction(&mut self) {
        let pc = self.state.get_pc();

        let opcode: word = match self.read(pc as _) {
            Some(o) => o,
            None => return,
        };

        self.prev_instruction = self.cur_instruction;
        self.cur_instruction = Some(Instruction::new(opcode));
    }

    fn execute_instruction(&mut self) {
        let Some(instr) = self.cur_instruction else {
            eprintln!("Tried to execute instruction before fetching");
            return;
        };

        let run_delay_slot = !self.delay_slot.is_empty();
        let run_instruction = if self.is_branch_likely {
            self.is_branch_likely = false;

            self.delay_slot[0].2
        } else {
            true
        };

        if run_instruction {
            if self.logging {
                println!(
                    "Executing instruction {:016X}: {:X?}",
                    self.state.get_pc(),
                    instr
                );
            }

            instr.execute(self);
        }

        if run_delay_slot {
            if let Some((delay_slot, target, cond)) = self.delay_slot.pop_front() {
                let Some(prev_instr) = self.prev_instruction else {
                    eprintln!("Tried to execute non-existent delay slot");
                    return;
                };
                delay_slot(&prev_instr, self, target, cond);
            } else {
                eprintln!("Delay slot function should exist");
            }
        } else {
            self.state.set_pc(self.state.get_pc().wrapping_add(4))
        }
    }

    pub fn throw_exception(&mut self, exception: Exception) {
        if exception.priority() > self.exception.priority() {
            self.exception = exception;
        } else {
            /*println!(
                "priority of {:?} is not higher than current ({:?})",
                exception, self.exception
            );*/
        }
    }

    fn handle_exception(&mut self) {
        // i think the interrupt enable handling is wrong
        // oh well
        if self.exception.exception == ExceptionType::None
            || !self
                .cop0
                .state
                .get_reg::<cop0::registers::Status>(cop0::Register::Status)
                .ie()
            || self
                .cop0
                .state
                .get_reg::<cop0::registers::Status>(cop0::Register::Status)
                .exl()
            || self
                .cop0
                .state
                .get_reg::<cop0::registers::Status>(cop0::Register::Status)
                .erl()
        {
            /*if self.exception.exception != ExceptionType::None {
                println!(
                    "do not handle exception: {:?}",
                    self.cop0
                        .state
                        .get_reg::<cop0::registers::Status>(cop0::Register::Status)
                );
            }*/
            return;
        }

        println!("Exception: {:?}", self.exception.exception);

        let mut status: cop0::registers::Status = self.cop0.state.get_reg(cop0::Register::Status);

        match self.exception.exception {
            ExceptionType::Trap => status.set_erl(true),
            _ => status.set_exl(true),
        }

        self.cop0.state.set_reg(cop0::Register::Status, status);

        match self.exception.exception {
            ExceptionType::ColdReset => {
                self.did_cold_reset = true;
                self.cop0 = Cop0::new(
                    ResetType::Cold,
                    self.cop0.retrieve_bootrom(),
                    self.cop0.retrieve_v0(),
                    self.cop0.retrieve_v1(),
                    self.cop0.retrieve_v2(),
                    self.cop0.retrieve_nand(),
                    self.cop0.retrieve_spare(),
                );
                self.state = State::new();
                self.state.set_pc(Self::RESET_PC);
            }

            ExceptionType::SoftReset | ExceptionType::NMI => {
                if self.exception.exception == ExceptionType::SoftReset {
                    self.did_soft_reset = true;
                } else {
                    self.did_nmi = true;
                }

                self.cop0.state.set_reg(
                    cop0::Register::ErrorEpc,
                    cop0::registers::ErrorEpc::new().with_error_epc(self.state.get_pc() as word),
                );

                self.state.set_pc(Self::RESET_PC);
            }

            ExceptionType::Trap => {
                let mut epc = self.state.get_pc() as word;

                if !self.delay_slot.is_empty() {
                    epc = epc.wrapping_sub(4);

                    let mut cause: cop0::registers::Cause =
                        self.cop0.state.get_reg(cop0::Register::Cause);
                    cause.set_bd(true);
                    self.cop0.state.set_reg(cop0::Register::Cause, cause);

                    self.delay_slot.clear();
                    self.is_branch_likely = false;
                }

                self.cop0.state.set_reg(
                    cop0::Register::ErrorEpc,
                    cop0::registers::ErrorEpc::new().with_error_epc(epc),
                );

                self.state.set_pc(Self::SK_ENTER);
            }

            _ => {
                let mut epc = self.state.get_pc() as word;

                if !self.delay_slot.is_empty() {
                    epc = epc.wrapping_sub(4);

                    let mut cause: cop0::registers::Cause =
                        self.cop0.state.get_reg(cop0::Register::Cause);
                    cause.set_bd(true);
                    self.cop0.state.set_reg(cop0::Register::Cause, cause);

                    self.delay_slot.clear();
                    self.is_branch_likely = false;
                }

                self.cop0.state.set_reg(
                    cop0::Register::Epc,
                    cop0::registers::Epc::new().with_epc(epc),
                );

                self.state.set_pc(
                    if status.bev() {
                        Self::EXCEPTION_PC_BEV
                    } else {
                        Self::EXCEPTION_PC
                    } + if matches!(
                        self.exception.exception,
                        ExceptionType::TLBMissRead | ExceptionType::TLBMissWrite
                    ) {
                        Self::TLB_MISS_ADD
                    } else {
                        Self::OTHER_ADD
                    },
                );
            }
        }

        self.exception = Exception::default();
    }

    pub fn secure_trap(&mut self, trap: SecureTrapType) {
        self.throw_exception(Exception::new(ExceptionType::Trap));
        self.cop0.set_secure_trap(trap);
    }

    pub fn get_pc(&self) -> dword {
        self.state.get_pc()
    }

    pub fn get_bootram(&self) -> &[byte] {
        self.cop0.get_bootram()
    }

    pub fn get_mi_mapping(&self) -> bool {
        self.cop0.get_mi_mapping()
    }

    pub fn get_reg(&self, reg: byte) -> dword {
        self.state.get_reg(reg.into())
    }
}
