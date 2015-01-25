use oplib;
use cstate;
use modrm;
use datatypes::{Byte, Word};


pub enum Operand {
    RawByte(Byte),
    RawWord(Word),
    Modrm(modrm::ModrmResult),
    Reg8(cstate::Reg8),
    Reg16(cstate::Reg16),
    MemoryAddress(Word),
}


pub struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub sign: bool,
    pub zero: bool,
}


pub fn b_operand_value(cs: &mut cstate::CpuState, o: &Operand) -> Byte {
    return match *o {
        Operand::RawByte(ref v) => *v,
        Operand::RawWord(ref v) => panic!("invalid"),
        Operand::Modrm(ref v) => oplib::modrm_value_b(cs, v),
        Operand::Reg8(ref reg) => cs.getreg_b(reg),
        Operand::Reg16(ref reg) => panic!("invalid"),
        Operand::MemoryAddress(ref addr) => cs.getmem(*addr),
    }
}

pub fn b_operand_set(cs: &mut cstate::CpuState, o: &Operand, result: Byte) {
    match *o {
        Operand::RawByte(ref v) => panic!("invalid"),
        Operand::RawWord(ref v) => panic!("invalid"),
        Operand::Modrm(ref v) => oplib::modrm_set_b(cs, v, result),
        Operand::Reg8(ref reg) => cs.setreg_b(reg, result),
        Operand::Reg16(ref reg) => cs.setreg_w(reg, result.to_u16().unwrap()),
        Operand::MemoryAddress(ref addr) => cs.setmem(*addr, result),
    }
}
