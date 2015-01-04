use cstate::{CpuState, Reg8, Reg16};
use datatypes::Byte;
use debugger;
use modrm;
use operations;


type F = fn(&mut CpuState, u8);

pub fn do_opcode(cs: &mut CpuState, opcode: Byte) {
    // TODO - Don't duplicate opcode definitions here and in their do_* method

    let func: F = match opcode {
        // Opcodes with immediate byte arguments
        0x04 | 0xB0 | 0xB1 | 0xB2 | 0xB3 | 0xB4 | 0xB5 | 0xB6 | 0xB7 | 0x3C | 0xEB | 0x74 => do_opcode_ib,

        // Opcodes with immediate word arguments
        0x05 | 0xB8 | 0xB9 | 0xBA | 0xBB | 0xBC | 0xBD | 0xBE | 0xBF | 0x3D | 0xE8 | 0xE9 => do_opcode_iw,

        // Opcodes with ModR/M arguments (operate on bytes)
        0x88 | 0x8A | 0x38 => do_opcode_mb,

        // Opcodes with ModR/M arguments (operate on words)
        0x89 | 0x8B | 0xC6 => do_opcode_mw,

        // Opcodes with no arguments
        0x40...0x47 | 0x50...0x5F | 0xC3 => do_opcode_none,

        // Group opcodes with immediate arguments (operate on bytes)
        0x80 => do_group,

        // Special opcodes
        0xF4 | 0x90 => do_special,

        _ => panic!("Unrecognized opcode: 0x{:X}", opcode),
    };

    func(cs, opcode);
}

/**
 * Handle operations with immediate byte arguments
 */
fn do_opcode_ib(cs: &mut CpuState, opcode: Byte) {
    let immediate = cs.read_b();

    match opcode {
        0x04 => operations::b_add(cs, Reg8::AL, immediate),

        0x74 => operations::jz(cs, immediate),

        0x3C => operations::b_cmp_ri(cs, Reg8::AL, immediate),

        0xB0 => operations::b_mov_ir(cs, Reg8::AL, immediate),
        0xB1 => operations::b_mov_ir(cs, Reg8::CL, immediate),
        0xB2 => operations::b_mov_ir(cs, Reg8::DL, immediate),
        0xB3 => operations::b_mov_ir(cs, Reg8::BL, immediate),
        0xB4 => operations::b_mov_ir(cs, Reg8::AH, immediate),
        0xB5 => operations::b_mov_ir(cs, Reg8::CH, immediate),
        0xB6 => operations::b_mov_ir(cs, Reg8::DH, immediate),
        0xB7 => operations::b_mov_ir(cs, Reg8::BH, immediate),

        0xEB => operations::b_jmp(cs, immediate),

        _ => panic!("Invalid opcode for do_opcode_ib: 0x{:X}", opcode),
    };
}

/**
 * Handle operations with immediate word arguments
 */
fn do_opcode_iw(cs: &mut CpuState, opcode: Byte) {
    let immediate = cs.read_w();

    match opcode {
        0x05 => operations::w_add(cs, Reg16::AX, immediate),

        0x3D => operations::w_cmp_ri(cs, Reg16::AX, immediate),

        0xB8 => operations::w_mov_ir(cs, Reg16::AX, immediate),
        0xB9 => operations::w_mov_ir(cs, Reg16::CX, immediate),
        0xBA => operations::w_mov_ir(cs, Reg16::DX, immediate),
        0xBB => operations::w_mov_ir(cs, Reg16::BX, immediate),
        0xBC => operations::w_mov_ir(cs, Reg16::SP, immediate),
        0xBE => operations::w_mov_ir(cs, Reg16::SI, immediate),
        0xBF => operations::w_mov_ir(cs, Reg16::DI, immediate),

        0xE8 => operations::call(cs, immediate),
        0xE9 => operations::w_jmp(cs, immediate),

        _ => panic!("Invalid opcode for do_opcode_iw: 0x{:X}", opcode),
    };
}

/**
 * Handle operations with ModR/M arguments (byte effective / register values)
 */
fn do_opcode_mb(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_b();
    let mb = modrm::ModrmByte::read(mb);
    let effective = mb.effective();
    let register = mb.register();

    match opcode {
        0x88 => operations::b_mov_eg(cs, effective, register),
        0x8A => operations::b_mov_ge(cs, effective, register),
        0x38 => operations::b_cmp_eg(cs, effective, register),

        _ => panic!("Invalid opcode for do_opcode_mb: 0x{:X}", opcode),
    };
}

/**
 * Handle operations with ModR/M arguments (word effective / register values)
 */
fn do_opcode_mw(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_b();
    let mb = modrm::ModrmByte::read(mb);
    let effective = mb.effective();
    let register = mb.register();

    match opcode {
        0x89 => operations::w_mov_eg(cs, effective, register),
        0x8B => operations::w_mov_ge(cs, effective, register),
        0xC6 => operations::mov_e(cs, effective, register),

        _ => panic!("Invalid opcode for do_opcode_mw: 0x{:X}", opcode),
    };
}


/**
 * Handle operations that take no arguments or for which the argument
 * is encoded in the opcode itself.
 */
fn do_opcode_none(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0x40 => operations::inc(cs, Reg16::AX),
        0x41 => operations::inc(cs, Reg16::CX),
        0x42 => operations::inc(cs, Reg16::DX),
        0x43 => operations::inc(cs, Reg16::BX),
        0x47 => operations::inc(cs, Reg16::DI),

        0x50 => operations::push(cs, Reg16::AX),
        0x51 => operations::push(cs, Reg16::CX),
        0x52 => operations::push(cs, Reg16::DX),
        0x53 => operations::push(cs, Reg16::BX),
        0x54 => operations::push(cs, Reg16::SP),
        0x56 => operations::push(cs, Reg16::SI),
        0x57 => operations::push(cs, Reg16::DI),

        0x58 => operations::pop(cs, Reg16::AX),
        0x59 => operations::pop(cs, Reg16::CX),
        0x5A => operations::pop(cs, Reg16::DX),
        0x5B => operations::pop(cs, Reg16::BX),
        0x5C => operations::pop(cs, Reg16::SP),
        0x5E => operations::pop(cs, Reg16::SI),
        0x5F => operations::pop(cs, Reg16::DI),

        0xC3 => operations::ret(cs),

        _ => panic!("Invalid opcode for do_opcode_none: 0x{:X}", opcode),
    };
}

/**
 * Handle group operations
 */
fn do_group(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x80 {
        panic!("Invalid opcode for do_opcode_group_eib: 0x{:X}", opcode);
    }

    let mb = cs.read_b();
    let mb = modrm::ModrmByte::read(mb);
    let effective = mb.effective(); 

    match mb.reg {
        0b111 => operations::b_cmp_ei(cs, effective),
        _ => panic!("Not Implemented"),
    }
}


/**
 * Handle special opcodes
 */
fn do_special(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0xF4 => {
            debugger::dump_state(cs);
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Invalid opcode for do_special: 0x{:X}", opcode),
    };
}
