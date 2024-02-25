use crate::types::*;
use crate::R4300i;

use modular_bitfield::prelude::*;

pub mod execute;

use execute::get_instruction_function;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct RFormat {
    function: B6,
    shift_amt: B5,
    dest: B5,
    source2: B5,
    source1: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct IFormat {
    imm: hword,
    source2: B5,
    source1: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct JFormat {
    target: B26,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct C0Format {
    sel: B3,
    #[skip]
    __: B8,
    dest: B5,
    source: B5,
    format: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FlsFormat {
    offset: hword,
    source: B5,
    base: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FmFormat {
    #[skip]
    __: B11,
    fpr: B5,
    gpr: B5,
    format: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FbFormat {
    offset: hword,
    ndtf: B2,
    cc: B3,
    format: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FrFormat {
    function: B6,
    dest: B5,
    source1: B5,
    source2: B5,
    format: B5,
    opcode: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FcFormat {
    cond: B4,
    #[skip]
    __: B4,
    cc: B3,
    source1: B5,
    source2: B5,
    format: B5,
    opcode: B6,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    None,

    Sll(RFormat),
    Srl(RFormat),
    Sra(RFormat),
    Sllv(RFormat),
    Srlv(RFormat),
    Srav(RFormat),
    Jr(RFormat),
    Jalr(RFormat),
    Syscall(RFormat),
    Break(RFormat),
    Sync(RFormat),
    Mfhi(RFormat),
    Mthi(RFormat),
    Mflo(RFormat),
    Mtlo(RFormat),
    Dsllv(RFormat),
    Dsrlv(RFormat),
    Dsrav(RFormat),
    Mult(RFormat),
    Multu(RFormat),
    Div(RFormat),
    Divu(RFormat),
    Dmult(RFormat),
    Dmultu(RFormat),
    Ddiv(RFormat),
    Ddivu(RFormat),
    Add(RFormat),
    Addu(RFormat),
    Sub(RFormat),
    Subu(RFormat),
    And(RFormat),
    Or(RFormat),
    Xor(RFormat),
    Nor(RFormat),
    Slt(RFormat),
    Sltu(RFormat),
    Dadd(RFormat),
    Daddu(RFormat),
    Dsub(RFormat),
    Dsubu(RFormat),
    Tge(RFormat),
    Tgeu(RFormat),
    Tlt(RFormat),
    Tltu(RFormat),
    Teq(RFormat),
    Tne(RFormat),
    Dsll(RFormat),
    Dsrl(RFormat),
    Dsra(RFormat),
    Dsll32(RFormat),
    Dsrl32(RFormat),
    Dsra32(RFormat),

    Bltz(IFormat),
    Bgez(IFormat),
    Bltzl(IFormat),
    Bgezl(IFormat),
    Tgei(IFormat),
    Tgeiu(IFormat),
    Tlti(IFormat),
    Tltiu(IFormat),
    Teqi(IFormat),
    Tnei(IFormat),
    Bltzal(IFormat),
    Bgezal(IFormat),
    Bltzall(IFormat),
    Bgezall(IFormat),

    Mfc0(C0Format),
    Mtc0(C0Format),

    Mfc1(FmFormat),
    Dmfc1(FmFormat),
    Cfc1(FmFormat),
    Mtc1(FmFormat),
    Dmtc1(FmFormat),
    Ctc1(FmFormat),

    Bc1f(FbFormat),
    Bc1t(FbFormat),
    Bc1fl(FbFormat),
    Bc1tl(FbFormat),

    Addf(FrFormat),
    Subf(FrFormat),
    Mulf(FrFormat),
    Divf(FrFormat),
    Sqrtf(FrFormat),
    Absf(FrFormat),
    Movf(FrFormat),
    Negf(FrFormat),
    Roundl(FrFormat),
    Truncl(FrFormat),
    Ceill(FrFormat),
    Floorl(FrFormat),
    Roundw(FrFormat),
    Truncw(FrFormat),
    Ceilw(FrFormat),
    Floorw(FrFormat),
    Cvts(FrFormat),
    Cvtd(FrFormat),
    Cvtw(FrFormat),
    Cvtl(FrFormat),
    Fcompare(FrFormat),

    J(JFormat),
    Jal(JFormat),

    Beq(IFormat),
    Bne(IFormat),
    Blez(IFormat),
    Bgtz(IFormat),
    Addi(IFormat),
    Addiu(IFormat),
    Slti(IFormat),
    Sltiu(IFormat),
    Andi(IFormat),
    Ori(IFormat),
    Xori(IFormat),
    Lui(IFormat),
    Beql(IFormat),
    Bnel(IFormat),
    Blezl(IFormat),
    Bgtzl(IFormat),
    Daddi(IFormat),
    Daddiu(IFormat),
    Ldl(IFormat),
    Ldr(IFormat),
    Lb(IFormat),
    Lh(IFormat),
    Lwl(IFormat),
    Lw(IFormat),
    Lbu(IFormat),
    Lhu(IFormat),
    Lwr(IFormat),
    Lwu(IFormat),
    Sb(IFormat),
    Sh(IFormat),
    Swl(IFormat),
    Sw(IFormat),
    Sdl(IFormat),
    Sdr(IFormat),
    Swr(IFormat),
    Cache(IFormat),
    Ll(IFormat),
    Lwc1(IFormat),
    Lld(IFormat),
    Ldc1(IFormat),
    Ld(IFormat),
    Sc(IFormat),
    Swc1(IFormat),
    Scd(IFormat),
    Sdc1(IFormat),
    Sd(IFormat),
}

const SPECIAL_OPCODE_TABLE: [Option<fn(RFormat) -> Instruction>; 64] = [
    Some(Instruction::Sll),
    None,
    Some(Instruction::Srl),
    Some(Instruction::Sra),
    Some(Instruction::Sllv),
    None,
    Some(Instruction::Srlv),
    Some(Instruction::Srav),
    Some(Instruction::Jr),
    Some(Instruction::Jalr),
    None,
    None,
    Some(Instruction::Syscall),
    Some(Instruction::Break),
    None,
    Some(Instruction::Sync),
    Some(Instruction::Mfhi),
    Some(Instruction::Mthi),
    Some(Instruction::Mflo),
    Some(Instruction::Mtlo),
    Some(Instruction::Dsllv),
    None,
    Some(Instruction::Dsrlv),
    Some(Instruction::Dsrav),
    Some(Instruction::Mult),
    Some(Instruction::Multu),
    Some(Instruction::Div),
    Some(Instruction::Divu),
    Some(Instruction::Dmult),
    Some(Instruction::Dmultu),
    Some(Instruction::Ddiv),
    Some(Instruction::Ddivu),
    Some(Instruction::Add),
    Some(Instruction::Addu),
    Some(Instruction::Sub),
    Some(Instruction::Subu),
    Some(Instruction::And),
    Some(Instruction::Or),
    Some(Instruction::Xor),
    Some(Instruction::Nor),
    None,
    None,
    Some(Instruction::Slt),
    Some(Instruction::Sltu),
    Some(Instruction::Dadd),
    Some(Instruction::Daddu),
    Some(Instruction::Dsub),
    Some(Instruction::Dsubu),
    Some(Instruction::Tge),
    Some(Instruction::Tgeu),
    Some(Instruction::Tlt),
    Some(Instruction::Tltu),
    Some(Instruction::Teq),
    None,
    Some(Instruction::Tne),
    None,
    Some(Instruction::Dsll),
    None,
    Some(Instruction::Dsrl),
    Some(Instruction::Dsra),
    Some(Instruction::Dsll32),
    None,
    Some(Instruction::Dsrl32),
    Some(Instruction::Dsra32),
];

const REGIMM_OPCODE_TABLE: [Option<fn(IFormat) -> Instruction>; 32] = [
    Some(Instruction::Bltz),
    Some(Instruction::Bgez),
    Some(Instruction::Bltzl),
    Some(Instruction::Bgezl),
    None,
    None,
    None,
    None,
    Some(Instruction::Tgei),
    Some(Instruction::Tgeiu),
    Some(Instruction::Tlti),
    Some(Instruction::Tltiu),
    Some(Instruction::Teqi),
    None,
    Some(Instruction::Tnei),
    None,
    Some(Instruction::Bltzal),
    Some(Instruction::Bgezal),
    Some(Instruction::Bltzall),
    Some(Instruction::Bgezall),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const COP0_OPCODE_TABLE: [Option<fn(C0Format) -> Instruction>; 32] = [
    Some(Instruction::Mfc0),
    None,
    None,
    None,
    Some(Instruction::Mtc0),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const FPU_OPCODE_TABLE: [Option<fn(FmFormat) -> Instruction>; 32] = [
    Some(Instruction::Mfc1),
    Some(Instruction::Dmfc1),
    Some(Instruction::Cfc1),
    None,
    Some(Instruction::Mtc1),
    Some(Instruction::Dmtc1),
    Some(Instruction::Ctc1),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const FPU_BC_OPCODE_TABLE: [fn(FbFormat) -> Instruction; 4] = [
    Instruction::Bc1f,
    Instruction::Bc1t,
    Instruction::Bc1fl,
    Instruction::Bc1tl,
];

const FPU_SDWL_OPCODE_TABLE: [Option<fn(FrFormat) -> Instruction>; 64] = [
    Some(Instruction::Addf),
    Some(Instruction::Subf),
    Some(Instruction::Mulf),
    Some(Instruction::Divf),
    Some(Instruction::Sqrtf),
    Some(Instruction::Absf),
    Some(Instruction::Movf),
    Some(Instruction::Negf),
    Some(Instruction::Roundl),
    Some(Instruction::Truncl),
    Some(Instruction::Ceill),
    Some(Instruction::Floorl),
    Some(Instruction::Roundw),
    Some(Instruction::Truncw),
    Some(Instruction::Ceilw),
    Some(Instruction::Floorw),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Instruction::Cvts),
    Some(Instruction::Cvtd),
    None,
    None,
    Some(Instruction::Cvtw),
    Some(Instruction::Cvtl),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
    Some(Instruction::Fcompare),
];

const OPCODE_TABLE: [Option<fn(IFormat) -> Instruction>; 64] = [
    None,
    None,
    None,
    None,
    Some(Instruction::Beq),
    Some(Instruction::Bne),
    Some(Instruction::Blez),
    Some(Instruction::Bgtz),
    Some(Instruction::Addi),
    Some(Instruction::Addiu),
    Some(Instruction::Slti),
    Some(Instruction::Sltiu),
    Some(Instruction::Andi),
    Some(Instruction::Ori),
    Some(Instruction::Xori),
    Some(Instruction::Lui),
    None,
    None,
    None,
    None,
    Some(Instruction::Beql),
    Some(Instruction::Bnel),
    Some(Instruction::Blezl),
    Some(Instruction::Bgtzl),
    Some(Instruction::Daddi),
    Some(Instruction::Daddiu),
    Some(Instruction::Ldl),
    Some(Instruction::Ldr),
    None,
    None,
    None,
    None,
    Some(Instruction::Lb),
    Some(Instruction::Lh),
    Some(Instruction::Lwl),
    Some(Instruction::Lw),
    Some(Instruction::Lbu),
    Some(Instruction::Lhu),
    Some(Instruction::Lwr),
    Some(Instruction::Lwu),
    Some(Instruction::Sb),
    Some(Instruction::Sh),
    Some(Instruction::Swl),
    Some(Instruction::Sw),
    Some(Instruction::Sdl),
    Some(Instruction::Sdr),
    Some(Instruction::Swr),
    Some(Instruction::Cache),
    Some(Instruction::Ll),
    Some(Instruction::Lwc1),
    None,
    None,
    Some(Instruction::Lld),
    Some(Instruction::Ldc1),
    None,
    Some(Instruction::Ld),
    Some(Instruction::Sc),
    Some(Instruction::Swc1),
    None,
    None,
    Some(Instruction::Scd),
    Some(Instruction::Sdc1),
    None,
    Some(Instruction::Sd),
];

impl Instruction {
    const OPC_SPECIAL: byte = 0o000;
    const OPC_REGIMM: byte = 0o001;
    const OPC_COP0: byte = 0o020;
    const OPC_COP1: byte = 0o021;
    const OPC_COP2: byte = 0o022;

    fn new_special(value: word) -> Self {
        let dec = RFormat::from(value);
        assert_eq!(dec.opcode(), Self::OPC_SPECIAL);

        match SPECIAL_OPCODE_TABLE[dec.function() as usize] {
            Some(i) => i(dec),
            None => Instruction::None,
        }
    }

    fn new_regimm(value: word) -> Self {
        let dec = IFormat::from(value);
        assert_eq!(dec.opcode(), Self::OPC_REGIMM);

        match REGIMM_OPCODE_TABLE[dec.source2() as usize] {
            Some(i) => i(dec),
            None => Instruction::None,
        }
    }

    fn new_cop0(value: word) -> Self {
        let dec = C0Format::from(value);
        assert_eq!(dec.opcode(), Self::OPC_COP0);

        match COP0_OPCODE_TABLE[dec.format() as usize] {
            Some(i) => i(dec),
            None => Instruction::None,
        }
    }

    fn new_cop1(value: word) -> Self {
        const FPU_CODE_BC: byte = 8;
        const FPU_CODE_S: byte = 16;
        const FPU_CODE_D: byte = 17;
        const FPU_CODE_W: byte = 20;
        const FPU_CODE_L: byte = 21;

        let dec = FmFormat::from(value);
        assert_eq!(dec.opcode(), Self::OPC_COP1);

        match dec.format() {
            FPU_CODE_BC => {
                let dec = FbFormat::from(value);

                FPU_BC_OPCODE_TABLE[dec.ndtf() as usize](dec)
            }

            FPU_CODE_S | FPU_CODE_D | FPU_CODE_W | FPU_CODE_L => {
                let dec = FrFormat::from(value);

                let instr = match FPU_SDWL_OPCODE_TABLE[dec.function() as usize] {
                    Some(i) => i(dec),
                    None => Instruction::None,
                };

                if (!matches!(instr, Instruction::Cvts(_) | Instruction::Cvtd(_))
                    && (dec.format() != FPU_CODE_W && dec.format() != FPU_CODE_L))
                    || (matches!(instr, Instruction::Cvts(_)) && dec.format() == FPU_CODE_S)
                    || (matches!(instr, Instruction::Cvtd(_)) && dec.format() == FPU_CODE_D)
                {
                    Instruction::None
                } else {
                    instr
                }
            }

            _ => match FPU_OPCODE_TABLE[dec.format() as usize] {
                Some(i) => i(dec),
                None => Instruction::None,
            },
        }
    }

    fn new_default(value: word) -> Self {
        let dec = IFormat::from(value);

        match dec.opcode() {
            2 => Instruction::J(JFormat::from(value)),
            3 => Instruction::Jal(JFormat::from(value)),
            _ => match OPCODE_TABLE[dec.opcode() as usize] {
                Some(i) => i(dec),
                None => Instruction::None,
            },
        }
    }

    pub fn new(value: word) -> Self {
        let opcode = (value >> 26) as u8;

        match opcode {
            Self::OPC_SPECIAL => Self::new_special(value),
            Self::OPC_REGIMM => Self::new_regimm(value),
            Self::OPC_COP0 => Self::new_cop0(value),
            Self::OPC_COP1 => Self::new_cop1(value),
            _ => Self::new_default(value),
        }
    }

    pub fn execute(&self, cpu: &mut R4300i) {
        get_instruction_function(self)(self, cpu);
    }
}
