use crate::types::*;
use crate::{Exception, ExceptionType, R4300i, Register};

use super::Instruction;

const DEFAULT_READ_VALUE: qword = 0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE;

pub type InstructionFunction = fn(&Instruction, &mut R4300i);

pub fn get_instruction_function(instr: &Instruction) -> InstructionFunction {
    match instr {
        Instruction::None => none,
        Instruction::Sll(_) => sll,
        Instruction::Srl(_) => srl,
        Instruction::Sra(_) => sra,
        Instruction::Sllv(_) => sllv,
        Instruction::Srlv(_) => srlv,
        Instruction::Srav(_) => todo!(),
        Instruction::Jr(_) => jr,
        Instruction::Jalr(_) => jalr,
        Instruction::Syscall(_) => todo!(),
        Instruction::Break(_) => todo!(),
        Instruction::Sync(_) => todo!(),
        Instruction::Mfhi(_) => mfhi,
        Instruction::Mthi(_) => todo!(),
        Instruction::Mflo(_) => mflo,
        Instruction::Mtlo(_) => todo!(),
        Instruction::Dsllv(_) => todo!(),
        Instruction::Dsrlv(_) => todo!(),
        Instruction::Dsrav(_) => todo!(),
        Instruction::Mult(_) => mult,
        Instruction::Multu(_) => todo!(),
        Instruction::Div(_) => div,
        Instruction::Divu(_) => divu,
        Instruction::Dmult(_) => todo!(),
        Instruction::Dmultu(_) => todo!(),
        Instruction::Ddiv(_) => todo!(),
        Instruction::Ddivu(_) => todo!(),
        Instruction::Add(_) => add,
        Instruction::Addu(_) => addu,
        Instruction::Sub(_) => todo!(),
        Instruction::Subu(_) => subu,
        Instruction::And(_) => and,
        Instruction::Or(_) => or,
        Instruction::Xor(_) => xor,
        Instruction::Nor(_) => nor,
        Instruction::Slt(_) => slt,
        Instruction::Sltu(_) => sltu,
        Instruction::Dadd(_) => todo!(),
        Instruction::Daddu(_) => daddu,
        Instruction::Dsub(_) => todo!(),
        Instruction::Dsubu(_) => todo!(),
        Instruction::Tge(_) => todo!(),
        Instruction::Tgeu(_) => todo!(),
        Instruction::Tlt(_) => todo!(),
        Instruction::Tltu(_) => todo!(),
        Instruction::Teq(_) => todo!(),
        Instruction::Tne(_) => todo!(),
        Instruction::Dsll(_) => dsll,
        Instruction::Dsrl(_) => todo!(),
        Instruction::Dsra(_) => todo!(),
        Instruction::Dsll32(_) => dsll32,
        Instruction::Dsrl32(_) => dsrl32,
        Instruction::Dsra32(_) => todo!(),
        Instruction::Bltz(_) => bltz,
        Instruction::Bgez(_) => bgez,
        Instruction::Bltzl(_) => bltzl,
        Instruction::Bgezl(_) => bgezl,
        Instruction::Tgei(_) => todo!(),
        Instruction::Tgeiu(_) => todo!(),
        Instruction::Tlti(_) => todo!(),
        Instruction::Tltiu(_) => todo!(),
        Instruction::Teqi(_) => todo!(),
        Instruction::Tnei(_) => todo!(),
        Instruction::Bltzal(_) => todo!(),
        Instruction::Bgezal(_) => todo!(),
        Instruction::Bltzall(_) => todo!(),
        Instruction::Bgezall(_) => todo!(),
        Instruction::Mfc0(_) => mfc0,
        Instruction::Mtc0(_) => mtc0,
        Instruction::Mfc1(_) => todo!(),
        Instruction::Dmfc1(_) => todo!(),
        Instruction::Cfc1(_) => todo!(),
        Instruction::Mtc1(_) => todo!(),
        Instruction::Dmtc1(_) => todo!(),
        Instruction::Ctc1(_) => todo!(),
        Instruction::Bc1f(_) => todo!(),
        Instruction::Bc1t(_) => todo!(),
        Instruction::Bc1fl(_) => todo!(),
        Instruction::Bc1tl(_) => todo!(),
        Instruction::Addf(_) => todo!(),
        Instruction::Subf(_) => todo!(),
        Instruction::Mulf(_) => todo!(),
        Instruction::Divf(_) => todo!(),
        Instruction::Sqrtf(_) => todo!(),
        Instruction::Absf(_) => todo!(),
        Instruction::Movf(_) => todo!(),
        Instruction::Negf(_) => todo!(),
        Instruction::Roundl(_) => todo!(),
        Instruction::Truncl(_) => todo!(),
        Instruction::Ceill(_) => todo!(),
        Instruction::Floorl(_) => todo!(),
        Instruction::Roundw(_) => todo!(),
        Instruction::Truncw(_) => todo!(),
        Instruction::Ceilw(_) => todo!(),
        Instruction::Floorw(_) => todo!(),
        Instruction::Cvts(_) => todo!(),
        Instruction::Cvtd(_) => todo!(),
        Instruction::Cvtw(_) => todo!(),
        Instruction::Cvtl(_) => todo!(),
        Instruction::Fcompare(_) => todo!(),
        Instruction::J(_) => j,
        Instruction::Jal(_) => jal,
        Instruction::Beq(_) => beq,
        Instruction::Bne(_) => bne,
        Instruction::Blez(_) => blez,
        Instruction::Bgtz(_) => bgtz,
        Instruction::Addi(_) => addi,
        Instruction::Addiu(_) => addiu,
        Instruction::Slti(_) => slti,
        Instruction::Sltiu(_) => sltiu,
        Instruction::Andi(_) => andi,
        Instruction::Ori(_) => ori,
        Instruction::Xori(_) => xori,
        Instruction::Lui(_) => lui,
        Instruction::Beql(_) => beql,
        Instruction::Bnel(_) => bnel,
        Instruction::Blezl(_) => todo!(),
        Instruction::Bgtzl(_) => todo!(),
        Instruction::Daddi(_) => todo!(),
        Instruction::Daddiu(_) => daddiu,
        Instruction::Ldl(_) => ldl,
        Instruction::Ldr(_) => ldr,
        Instruction::Lb(_) => lb,
        Instruction::Lh(_) => todo!(),
        Instruction::Lwl(_) => todo!(),
        Instruction::Lw(_) => lw,
        Instruction::Lbu(_) => lbu,
        Instruction::Lhu(_) => lhu,
        Instruction::Lwr(_) => todo!(),
        Instruction::Lwu(_) => todo!(),
        Instruction::Sb(_) => sb,
        Instruction::Sh(_) => sh,
        Instruction::Swl(_) => todo!(),
        Instruction::Sw(_) => sw,
        Instruction::Sdl(_) => sdl,
        Instruction::Sdr(_) => sdr,
        Instruction::Swr(_) => todo!(),
        Instruction::Cache(_) => cache,
        Instruction::Ll(_) => todo!(),
        Instruction::Lwc1(_) => todo!(),
        Instruction::Lld(_) => todo!(),
        Instruction::Ldc1(_) => todo!(),
        Instruction::Ld(_) => ld,
        Instruction::Sc(_) => todo!(),
        Instruction::Swc1(_) => todo!(),
        Instruction::Scd(_) => todo!(),
        Instruction::Sdc1(_) => todo!(),
        Instruction::Sd(_) => sd,
    }
}

macro_rules! get_instr {
    ($i:expr, $p:pat) => {
        match $i {
            $p(dec) => dec,
            _ => unreachable!(),
        }
    };
}

macro_rules! get_reg {
    ($c:expr, $s:expr, $t:ty) => {
        $c.state.get_reg($s.into()) as $t
    };
}

macro_rules! set_reg {
    ($c:expr, $s:expr, $v:expr) => {
        $c.state.set_reg($s.into(), $v)
    };
}

macro_rules! get_cop0_reg {
    ($c:expr, $s:expr, $t:ty) => {
        $c.cop0.state.get_reg_raw($s.into()) as $t
    };
}

macro_rules! set_cop0_reg {
    ($c:expr, $s:expr, $v:expr) => {
        $c.cop0.state.set_reg_raw($s.into(), $v)
    };
}

macro_rules! advance_pc {
    ($c:expr) => {
        $c.state.set_pc($c.state.get_pc().wrapping_add(4))
    };
}

macro_rules! link {
    ($c:expr, $s:expr, $o:expr) => {
        $c.state
            .set_reg($s.into(), $c.state.get_pc().wrapping_add($o))
    };
}

const BRANCH_FUNCTION: InstructionFunction = |_: &Instruction, cpu: &mut R4300i| {
    if cpu.delay_slot_condition {
        cpu.state
            .set_pc(cpu.state.get_pc().wrapping_add(cpu.delay_slot_target));
    } else {
        advance_pc!(cpu);
    }
};

const JUMP_FUNCTION: InstructionFunction = |_: &Instruction, cpu: &mut R4300i| {
    cpu.state
        .set_pc((cpu.state.get_pc() & 0xFFFFFFFF_F0000000) | (cpu.delay_slot_target << 2));
};

const JUMP_REGISTER_FUNCTION: InstructionFunction = |_: &Instruction, cpu: &mut R4300i| {
    cpu.state.set_pc(cpu.delay_slot_target);
};

fn none(instr: &Instruction, cpu: &mut R4300i) {
    cpu.throw_exception(Exception::new(ExceptionType::ReservedInstruction));
}

fn sll(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sll(dec) = instr else {
        unreachable!()
    };

    let source = lower_word(get_reg!(cpu, dec.source2(), _));
    let shift_amt = dec.shift_amt();

    set_reg!(cpu, dec.dest(), sign_extend_word(source << shift_amt));
}

fn srl(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Srl(dec) = instr else {
        unreachable!()
    };

    let source = lower_word(get_reg!(cpu, dec.source2(), _));
    let shift_amt = dec.shift_amt();

    set_reg!(cpu, dec.dest(), sign_extend_word(source >> shift_amt));
}

fn sra(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sra(dec) = instr else {
        unreachable!()
    };

    let source = lower_word(get_reg!(cpu, dec.source2(), _)) as sword;
    let shift_amt = dec.shift_amt();

    set_reg!(
        cpu,
        dec.dest(),
        sign_extend_word((source >> shift_amt) as _)
    );
}

fn sllv(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sllv(dec) = instr else {
        unreachable!()
    };

    let shift_amt = get_reg!(cpu, dec.source1(), u8) & 0x1F; // lower 5 bits
    let source = lower_word(get_reg!(cpu, dec.source2(), _));

    set_reg!(cpu, dec.dest(), sign_extend_word(source << shift_amt));
}

fn srlv(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Srlv(dec) = instr else {
        unreachable!()
    };

    let shift_amt = get_reg!(cpu, dec.source1(), u8) & 0x1F; // lower 5 bits
    let source = lower_word(get_reg!(cpu, dec.source2(), _));

    set_reg!(cpu, dec.dest(), sign_extend_word(source >> shift_amt));
}

fn jr(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Jr(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = get_reg!(cpu, dec.source1(), _);

    cpu.delay_slot = Some(JUMP_REGISTER_FUNCTION);
}

fn jalr(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Jalr(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = get_reg!(cpu, dec.source1(), _);

    link!(cpu, dec.dest(), 8);

    cpu.delay_slot = Some(JUMP_REGISTER_FUNCTION);
}

fn mfhi(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Mfhi(dec) = instr else {
        unreachable!()
    };

    set_reg!(cpu, dec.dest(), cpu.state.get_hi());
}

fn mflo(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Mflo(dec) = instr else {
        unreachable!()
    };

    set_reg!(cpu, dec.dest(), cpu.state.get_lo());
}

fn mult(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Mult(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), sword);
    let source2 = get_reg!(cpu, dec.source2(), sword);

    let result = source1 as sdword * source2 as sdword;

    cpu.state.set_lo(sign_extend_word(lower_word(result as _)));
    cpu.state.set_hi(sign_extend_word(upper_word(result as _)));
}

fn div(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Div(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), sword);
    let source2 = get_reg!(cpu, dec.source2(), sword);

    if source2 == 0 {
        return;
    }

    cpu.state
        .set_lo(sign_extend_word((source1 / source2) as word));
    cpu.state
        .set_hi(sign_extend_word((source1 % source2) as word));
}

fn divu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Divu(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), word);
    let source2 = get_reg!(cpu, dec.source2(), word);

    if source2 == 0 {
        return;
    }

    cpu.state.set_lo(sign_extend_word((source1 / source2) as _));
    cpu.state.set_hi(sign_extend_word((source1 % source2) as _));
}

fn add(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Add(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), sdword);
    let source2 = get_reg!(cpu, dec.source2(), sdword);

    let (add, overflow) = source1.overflowing_add(source2);
    let result = lower_word(add as _) as sword;

    if overflow {
        cpu.throw_exception(Exception::new(ExceptionType::ArithmeticOverflow));
    } else {
        set_reg!(cpu, dec.dest(), sign_extend_word(result as _));
    }
}

fn addu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Addu(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    let result = lower_word(source1.wrapping_add(source2));

    set_reg!(cpu, dec.dest(), sign_extend_word(result));
}

fn subu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Subu(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    let result = lower_word(source1.wrapping_sub(source2));

    set_reg!(cpu, dec.dest(), sign_extend_word(result));
}

fn and(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::And(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    set_reg!(cpu, dec.dest(), source1 & source2);
}

fn or(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Or(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    set_reg!(cpu, dec.dest(), source1 | source2);
}

fn xor(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Xor(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    set_reg!(cpu, dec.dest(), source1 ^ source2);
}

fn nor(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Nor(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    set_reg!(cpu, dec.dest(), !(source1 | source2));
}

fn slt(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Slt(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), sdword);
    let source2 = get_reg!(cpu, dec.source2(), sdword);

    set_reg!(cpu, dec.dest(), if source1 < source2 { 1 } else { 0 })
}

fn sltu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sltu(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    set_reg!(cpu, dec.dest(), if source1 < source2 { 1 } else { 0 })
}

fn daddu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Daddu(dec) = instr else {
        unreachable!()
    };

    let source1 = get_reg!(cpu, dec.source1(), dword);
    let source2 = get_reg!(cpu, dec.source2(), dword);

    let result = source1.wrapping_add(source2);

    set_reg!(cpu, dec.dest(), result);
}

fn dsll(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Dsll(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source2(), dword);
    let shift_amt = dec.shift_amt();

    set_reg!(cpu, dec.dest(), source << shift_amt);
}

fn dsll32(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Dsll32(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source2(), dword);
    let shift_amt = dec.shift_amt() + 32;

    set_reg!(cpu, dec.dest(), source << shift_amt);
}

fn dsrl32(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Dsrl32(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source2(), dword);
    let shift_amt = dec.shift_amt() + 32;

    set_reg!(cpu, dec.dest(), source >> shift_amt);
}

fn bltz(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bltz(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) < 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bgez(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bgez(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) >= 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bltzl(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bltzl(dec) = instr else {
        unreachable!()
    };

    cpu.is_branch_likely = true;

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) < 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bgezl(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bgezl(dec) = instr else {
        unreachable!()
    };

    cpu.is_branch_likely = true;

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) >= 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn mfc0(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Mfc0(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = get_cop0_reg!(cpu, dec.dest(), _);
    cpu.delay_slot_condition = true;

    cpu.delay_slot = Some(|instr: &Instruction, cpu: &mut R4300i| {
        let Instruction::Mfc0(dec) = instr else {
            unreachable!()
        };

        set_reg!(
            cpu,
            dec.source(),
            sign_extend_word(cpu.delay_slot_target as _)
        );
        advance_pc!(cpu);
    })
}

fn mtc0(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Mtc0(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = get_reg!(cpu, dec.source(), dword);
    cpu.delay_slot_condition = true;

    cpu.delay_slot = Some(|instr: &Instruction, cpu: &mut R4300i| {
        let Instruction::Mtc0(dec) = instr else {
            unreachable!()
        };

        set_cop0_reg!(cpu, dec.dest(), cpu.delay_slot_target as _);
        advance_pc!(cpu);
    });
}

fn j(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::J(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = dec.target() as _;

    cpu.delay_slot = Some(JUMP_FUNCTION);
}

fn jal(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Jal(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = dec.target() as _;

    link!(cpu, Register::Ra, 8);

    cpu.delay_slot = Some(JUMP_FUNCTION);
}

fn beq(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Beq(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition =
        get_reg!(cpu, dec.source1(), dword) == get_reg!(cpu, dec.source2(), dword);

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bne(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bne(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition =
        get_reg!(cpu, dec.source1(), dword) != get_reg!(cpu, dec.source2(), dword);

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn blez(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Blez(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) <= 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bgtz(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bgtz(dec) = instr else {
        unreachable!()
    };

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition = get_reg!(cpu, dec.source1(), sword) > 0;

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn addi(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Addi(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), sdword);
    let imm = sign_extend_hword_twice(dec.imm()) as sdword;

    let (add, overflow) = source.overflowing_add(imm);
    let result = lower_word(add as _) as sword;

    if overflow {
        cpu.throw_exception(Exception::new(ExceptionType::ArithmeticOverflow));
    } else {
        set_reg!(cpu, dec.source2(), sign_extend_word(result as _));
    }
}

fn addiu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Addiu(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), word);
    let imm = sign_extend_hword(dec.imm());

    let result = source.wrapping_add(imm);

    set_reg!(cpu, dec.source2(), sign_extend_word(result));
}

fn slti(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Slti(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), sdword);
    let imm = sign_extend_hword_twice(dec.imm()) as _;

    set_reg!(cpu, dec.source2(), if source < imm { 1 } else { 0 });
}

fn sltiu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sltiu(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), dword);
    let imm = sign_extend_hword_twice(dec.imm()) as _;

    set_reg!(cpu, dec.source2(), if source < imm { 1 } else { 0 });
}

fn andi(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Andi(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), dword);
    let imm = dec.imm() as dword;

    set_reg!(cpu, dec.source2(), source & imm);
}

fn ori(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Ori(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), dword);
    let imm = dec.imm() as dword;

    set_reg!(cpu, dec.source2(), source | imm);
}

fn xori(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Xori(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), dword);
    let imm = dec.imm() as dword;

    set_reg!(cpu, dec.source2(), source ^ imm);
}

fn lui(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Lui(dec) = instr else {
        unreachable!()
    };

    set_reg!(
        cpu,
        dec.source2(),
        sign_extend_word((dec.imm() as word) << 16)
    )
}

fn beql(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Beql(dec) = instr else {
        unreachable!()
    };

    cpu.is_branch_likely = true;

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition =
        get_reg!(cpu, dec.source1(), dword) == get_reg!(cpu, dec.source2(), dword);

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn bnel(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Bnel(dec) = instr else {
        unreachable!()
    };

    cpu.is_branch_likely = true;

    cpu.delay_slot_target = sign_extend_hword_twice(dec.imm()) << 2;
    cpu.delay_slot_condition =
        get_reg!(cpu, dec.source1(), dword) != get_reg!(cpu, dec.source2(), dword);

    cpu.delay_slot = Some(BRANCH_FUNCTION);
}

fn daddiu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Daddiu(dec) = instr else {
        unreachable!()
    };

    let source = get_reg!(cpu, dec.source1(), dword);
    let imm = sign_extend_hword_twice(dec.imm());

    let result = source.wrapping_add(imm);

    set_reg!(cpu, dec.source2(), result);
}

fn ldl(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Ldl(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let address = base.wrapping_add(offset) as word;
    let aligned_address = address & !7;
    let misalignment = address & 7;

    if misalignment != 0 {
        println!("l misaligned");
    }

    let mem = cpu
        .read::<dword>(aligned_address)
        .unwrap_or(DEFAULT_READ_VALUE as _);

    let reg = get_reg!(cpu, dec.source2(), dword);

    let val = if misalignment != 0 {
        (mem << (misalignment * 8)) | (reg & !((1 << ((8 - misalignment) * 8)) - 1))
    } else {
        mem
    };

    set_reg!(cpu, dec.source2(), val);
}

fn ldr(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Ldr(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let address = base.wrapping_add(offset) as word;
    let aligned_address = address & !7;
    let misalignment = address.wrapping_add(1) & 7;

    if misalignment != 0 {
        println!("r misaligned");
    }

    let mem = cpu
        .read::<dword>(aligned_address)
        .unwrap_or(DEFAULT_READ_VALUE as _);

    let reg = get_reg!(cpu, dec.source2(), dword);

    let val = if misalignment != 0 {
        (mem >> ((8 - misalignment) * 8)) | (reg & !((1 << (misalignment * 8)) - 1))
    } else {
        return;
    };

    set_reg!(cpu, dec.source2(), val);
}

fn lb(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Lb(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let val = sign_extend_byte_thrice(
        cpu.read(base.wrapping_add(offset) as _)
            .unwrap_or(DEFAULT_READ_VALUE as _),
    );
    set_reg!(cpu, dec.source2(), val);
}

fn lw(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Lw(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let val = sign_extend_word(
        cpu.read(base.wrapping_add(offset) as _)
            .unwrap_or(DEFAULT_READ_VALUE as _),
    );
    set_reg!(cpu, dec.source2(), val);
}

fn lbu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Lbu(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let val = cpu
        .read::<byte>(base.wrapping_add(offset) as _)
        .unwrap_or(DEFAULT_READ_VALUE as _);
    set_reg!(cpu, dec.source2(), val as _);
}

fn lhu(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Lhu(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let val = cpu
        .read::<hword>(base.wrapping_add(offset) as _)
        .unwrap_or(DEFAULT_READ_VALUE as _);
    set_reg!(cpu, dec.source2(), val as _);
}

fn sb(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sb(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    cpu.write(
        base.wrapping_add(offset) as _,
        get_reg!(cpu, dec.source2(), byte),
    );
}

fn sh(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sh(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    cpu.write(
        base.wrapping_add(offset) as _,
        get_reg!(cpu, dec.source2(), hword),
    );
}

fn sw(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sw(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    cpu.write(
        base.wrapping_add(offset) as _,
        get_reg!(cpu, dec.source2(), word),
    );
}

fn sdl(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sdl(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let address = base.wrapping_add(offset) as word;

    let val = get_reg!(cpu, dec.source2(), dword);

    for i in 0..(8 - (address & 7)) {
        cpu.write(address.wrapping_add(i), (val >> ((7 - i) * 8)) as byte);
    }
}

fn sdr(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sdr(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let address = base.wrapping_add(offset) as word;

    let val = get_reg!(cpu, dec.source2(), dword);

    for i in 0..((address.wrapping_add(1)) & 7) {
        cpu.write(address.wrapping_sub(i), (val >> (i * 8)) as byte);
    }
}

fn cache(instr: &Instruction, cpu: &mut R4300i) {
    eprintln!("CACHE operation unimplemented; skipping");
}

fn ld(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Ld(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    let val = cpu
        .read(base.wrapping_add(offset) as _)
        .unwrap_or(DEFAULT_READ_VALUE as _);

    set_reg!(cpu, dec.source2(), val);
}

fn sd(instr: &Instruction, cpu: &mut R4300i) {
    let Instruction::Sd(dec) = instr else {
        unreachable!()
    };

    let base = get_reg!(cpu, dec.source1(), dword);
    let offset = sign_extend_hword_twice(dec.imm());

    cpu.write(
        base.wrapping_add(offset) as _,
        get_reg!(cpu, dec.source2(), dword),
    );
}
