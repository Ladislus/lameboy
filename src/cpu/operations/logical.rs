use crate::cpu::instruction::VoidInstruction;
use crate::cpu::memory::Memory;
use crate::utils::bits::{assign_bit, get_bit, max_bit_index, bit_size, check_half_carry_sub};
use crate::utils::log::log;
use crate::utils::types::Void;

//  #############################
//  #         Template          #
//  #############################

macro_rules! template_and_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let result = old_value ^ value;

            $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(true);
            $memory.registers.set_carry_flag(false);
        }
    };
}

macro_rules! template_xor_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let result = old_value & value;

            $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(false);
        }
    };
}

macro_rules! template_or_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let result = old_value | value;

            $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(true);
            $memory.registers.set_carry_flag(false);
        }
    };
}

macro_rules! template_cp_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_sub(value);

            // $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(true);
            // H => Set if borrow from bit 4.
            $memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, value));
            // C => Set if overflow from bit 15.
            $memory.registers.set_carry_flag(has_overflown);
        }
    };
}

//  #############################
//  #           Rotate          #
//  #############################

pub fn rla(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_a();
    let old_carry = memory.registers.get_carry_flag();
    let popped_value = get_bit(old_value, max_bit_index(old_value));
    let new_value = assign_bit(old_value << 1, 0, old_carry);

    log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    memory.registers.set_a(new_value);
    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rlca(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    /*
        RLCA is "Rotate left circular register A"

        It shifts left the register A by one
        Popped value is place in carry flag & return to the lowest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, max_bit_index(old_value));
    // Left shift and put back top bit in the lowest bit
    let new_value = assign_bit(old_value << 1, 0, popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rra(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_a();
    let old_carry = memory.registers.get_carry_flag();
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), old_carry);

    log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rrca(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    /*
        RLCA is "Rotate right circular register A"

        It shifts right the register A by one
        Popped value is place in carry flag & return to the highest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    // Left shift and put back top bit in the lowest bit
    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

//  #############################
//  #          Bit And          #
//  #############################

pub fn and_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn and_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn and_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn and_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn and_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn and_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn and_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn and_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #          Bit Or           #
//  #############################

pub fn or_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn or_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn or_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn or_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn or_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn or_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn or_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn or_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #          Bit Xor          #
//  #############################

pub fn xor_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn xor_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn xor_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn xor_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn xor_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn xor_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn xor_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn xor_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #        Comparison         #
//  #############################

pub fn cp_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn cp_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn cp_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn cp_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn cp_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn cp_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn cp_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn cp_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}