use crate::cpu::memory::Memory;
use crate::utils::types::{FarAddress, Value, Void, WideValue};

//  #############################
//  #         Template          #
//  #############################

macro_rules! template_ld {
    ($field: expr, $value: ident) => {
        $field = $value;
    };
    ($field: expr, $value: expr) => {
        unsafe { $field = $value; }
    };
}

//  #############################
//  #       8-bits loads        #
//  #############################

//  ############# A #############

pub fn ld_a_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.AF.as_pair.0, value);
}

pub fn ld_a_a(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.AF.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_a_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_a_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_a_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_a_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_a_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_a_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_a_bc_addr(memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_bc()));
}

pub fn ld_a_de_addr(memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_de()));
}

pub fn ld_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_a_hli_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);

    memory.registers.set_hl(hl_value + 1);
    memory.registers.set_a(read_value);
}

pub fn ld_a_hld_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);

    memory.registers.set_hl(hl_value - 1);
    memory.registers.set_a(read_value);
}

//  ############# B #############

pub fn ld_b_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.BC.as_pair.0, value);
}

pub fn ld_b_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_b_b(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.BC.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_b_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_b_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_b_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_b_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_b_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_b_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############# C #############

pub fn ld_c_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.BC.as_pair.1, value);
}

pub fn ld_c_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_c_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_c_c(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.BC.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_c_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_c_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_c_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_c_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_c_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############ BC #############

pub fn ld_bc_addr_a(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_bc(), memory.registers.get_a());
}

//  ############# D #############

pub fn ld_d_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.DE.as_pair.0, value);
}

pub fn ld_d_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_d_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_d_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_d_d(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.DE.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_d_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_d_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_d_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_d_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############# E #############

pub fn ld_e_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.DE.as_pair.1, value);
}

pub fn ld_e_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_e_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_e_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_e_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_e_e(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.DE.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_e_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_e_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_e_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############ DE #############

pub fn ld_de_addr_a(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_de(), memory.registers.get_a());
}

//  ############# H #############

pub fn ld_h_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.HL.as_pair.0, value);
}

pub fn ld_h_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_h_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_h_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_h_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_h_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_h_h(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.HL.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_h_l(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_h_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############# L #############

pub fn ld_l_d8(memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.HL.as_pair.1, value);
}

pub fn ld_l_a(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_l_b(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_l_c(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_l_d(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_l_e(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_l_h(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_l_l(_memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.HL.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_l_hl_addr(memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

//  ############ HL #############


pub fn ld_hl_addr_d8(memory: &mut Memory, value: Value) {
    memory.write_far_addr(memory.registers.get_hl(), value);
}

pub fn ld_hl_addr_a(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_a());
}

pub fn ld_hli_addr_a(memory: &mut Memory, _value: Void) {
    // This is sometimes written as ‘LD (HLI),A’, or ‘LDI (HL),A’.
    let hl_value = memory.registers.get_hl();
    memory.write_far_addr(hl_value, memory.registers.get_a());
    memory.registers.set_hl(hl_value + 1);
}

pub fn ld_hld_addr_a(memory: &mut Memory, _value: Void) {
    // This is sometimes written as ‘LD (HLD),A’, or ‘LDD (HL),A’.
    let hl_value = memory.registers.get_hl();
    memory.write_far_addr(hl_value, memory.registers.get_a());
    memory.registers.set_hl(hl_value - 1);
}

pub fn ld_hl_addr_b(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_b());
}

pub fn ld_hl_addr_c(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_c());
}

pub fn ld_hl_addr_d(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_d());
}

pub fn ld_hl_addr_e(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_e());
}

pub fn ld_hl_addr_h(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_h());
}

pub fn ld_hl_addr_l(memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_l());
}

//  #############################
//  #       16-bits loads       #
//  #############################

//  ############ BC #############

pub fn ld_bc_d16(memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.BC.as_wide, value);
}

//  ############ DE #############

pub fn ld_de_d16(memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.DE.as_wide, value);
}

//  ############ HL #############

pub fn ld_hl_d16(memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.HL.as_wide, value);
}

//  ############ SP #############

pub fn ld_sp_d16(memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.SP, value);
}

pub fn ld_a16_addr_sp(memory: &mut Memory, value: FarAddress) {
    memory.write_wide_far_addr(value, memory.registers.SP);
}

//  #############################
//  #           Stack           #
//  #############################

//  ########### Push ############

// TODO: Check
pub fn push_bc(memory: &mut Memory, _value: Void) {
    let bc_value = memory.registers.get_bc();
    memory.stack.push_wide(&mut memory.registers.SP, bc_value);
}

// TODO: Check
pub fn push_de(memory: &mut Memory, _value: Void) {
    let de_value = memory.registers.get_de();
    memory.stack.push_wide(&mut memory.registers.SP, bc_value);
}

//  ########### Pop  ############

// TODO: Check
pub fn pop_bc(memory: &mut Memory, _value: Void) {
    let value = memory.stack.pop_wide(&mut memory.registers.SP);
    memory.registers.set_bc(value);
}

pub fn pop_de(memory: &mut Memory, _value: Void) {
    let value = memory.stack.pop_wide(&mut memory.registers.SP);
    memory.registers.set_de(value);
}