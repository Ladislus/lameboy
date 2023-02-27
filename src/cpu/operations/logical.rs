use crate::cpu::memory::Memory;
use crate::utils::bits::check_half_carry_sub;
use crate::utils::types::{Value, Void};

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
//  #          Bit And          #
//  #############################

pub fn and_a_d8(memory: &mut Memory, value: Value) {
    template_and_a!(memory, value);
}

pub fn and_a_a(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn and_a_b(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn and_a_c(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn and_a_d(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn and_a_e(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn and_a_h(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn and_a_l(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn and_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #          Bit Or           #
//  #############################

pub fn or_a_d8(memory: &mut Memory, value: Value) {
    template_or_a!(memory, value);
}

pub fn or_a_a(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn or_a_b(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn or_a_c(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn or_a_d(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn or_a_e(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn or_a_h(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn or_a_l(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn or_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #          Bit Xor          #
//  #############################

pub fn xor_a_d8(memory: &mut Memory, value: Value) {
    template_xor_a!(memory, value);
}

pub fn xor_a_a(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn xor_a_b(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn xor_a_c(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn xor_a_d(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn xor_a_e(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn xor_a_h(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn xor_a_l(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn xor_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #        Comparison         #
//  #############################

pub fn cp_a_d8(memory: &mut Memory, value: Value) {
    template_cp_a!(memory, value);
}

pub fn cp_a_a(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn cp_a_b(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn cp_a_c(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn cp_a_d(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn cp_a_e(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn cp_a_h(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn cp_a_l(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn cp_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}