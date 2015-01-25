use cstate::{CpuState, Reg8, Reg16};
use datatypes::{Byte, Word};
use modrm::ModrmResult;
use oplib;


// TODO - Replace with operand::Flags
pub type FlagFn = fn(&CpuState) -> bool;


pub fn ret(cs: &mut CpuState) {
    oplib::ret(cs);
}

pub fn xchg(cs: &mut CpuState, left: Reg16, right: Reg16) {
    let left_value = cs.getreg_w(&left);
    let right_value = cs.getreg_w(&right);
    cs.setreg_w(&left, right_value);
    cs.setreg_w(&right, left_value);
}

pub fn stc(cs: &mut CpuState) {
    cs.set_carry();
}

pub fn push(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    oplib::push(cs, cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    let popped_val = oplib::pop(cs);
    cs.setreg_w(&reg, popped_val);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    oplib::jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    oplib::jump_w(cs, immediate);
}

pub fn b_jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    oplib::jump_b(cs, immediate);
}

pub fn b_jmp_flags(cs: &mut CpuState, flag0_fn: FlagFn, flag1_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag0_value = flag0_fn(cs);
    let flag1_value = flag1_fn(cs);
    let flags_value = (flag0_fn(cs) || flag1_fn(cs));

    if !(flags_value ^ invert) {
        return;
    }

    oplib::jump_b(cs, immediate);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    oplib::call(cs, immediate);
}

pub fn b_xchg_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_b(cs, &left);
    let right_value = oplib::modrm_value_b(cs, &right);
    oplib::modrm_set_b(cs, &left, right_value);
    oplib::modrm_set_b(cs, &right, left_value);
}
