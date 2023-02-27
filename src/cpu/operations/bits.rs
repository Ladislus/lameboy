use crate::cpu::memory::Memory;
use crate::utils::bits::{assign_bit, bit_size, get_bit, max_bit_index};
use crate::utils::log::log;
use crate::utils::types::{FarAddress, Void};

//  #############################
//  #         Template          #
//  #############################

macro_rules! template_rl {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let old_carry = $memory.registers.get_carry_flag();
            let popped_value = get_bit(old_value, max_bit_index(old_value));
            let new_value = assign_bit(old_value << 1, 0, old_carry);

            log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_rlc {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let popped_value = get_bit(old_value, max_bit_index(old_value));
            // Left shift and put back top bit in the lowest bit
            let new_value = assign_bit(old_value << 1, 0, popped_value);

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_rr {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let old_carry = $memory.registers.get_carry_flag();
            let popped_value = get_bit(old_value, 0);
            let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), old_carry);

            log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_rrc {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let popped_value = get_bit(old_value, 0);
            let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), popped_value);

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            // Left shift and put back top bit in the lowest bit
            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

//  #############################
//  #           Rotate          #
//  #############################

//  ############# A #############

pub fn rla(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.AF.as_pair.0);
    memory.registers.set_zero_flag(false);
}

pub fn rlca(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.AF.as_pair.0);
    memory.registers.set_zero_flag(false);
}

pub fn rra(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.AF.as_pair.0);
    memory.registers.set_zero_flag(false);
}

pub fn rrca(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.AF.as_pair.0);
    memory.registers.set_zero_flag(false);
}

pub fn rl_a(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.AF.as_pair.0);
}

pub fn rlc_a(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.AF.as_pair.0);
}

pub fn rr_a(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.AF.as_pair.0);
}

pub fn rrc_a(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.AF.as_pair.0);
}

//  ############# B #############

pub fn rl_b(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.BC.as_pair.0);
}

pub fn rlc_b(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.BC.as_pair.0);
}

pub fn rr_b(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.BC.as_pair.0);
}

pub fn rrc_b(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.BC.as_pair.0);
}

//  ############# C #############

pub fn rl_c(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.BC.as_pair.1);
}

pub fn rlc_c(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.BC.as_pair.1);
}

pub fn rr_c(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.BC.as_pair.1);
}

pub fn rrc_c(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.BC.as_pair.1);
}

//  ############# D #############

pub fn rl_d(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.DE.as_pair.0);
}

pub fn rlc_d(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.DE.as_pair.0);
}

pub fn rr_d(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.DE.as_pair.0);
}

pub fn rrc_d(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.DE.as_pair.0);
}

//  ############# E #############

pub fn rl_e(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.DE.as_pair.1);
}

pub fn rlc_e(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.DE.as_pair.1);
}

pub fn rr_e(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.DE.as_pair.1);
}

pub fn rrc_e(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.DE.as_pair.1);
}

//  ############# H #############

pub fn rl_h(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.HL.as_pair.0);
}

pub fn rlc_h(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.HL.as_pair.0);
}

pub fn rr_h(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.HL.as_pair.0);
}

pub fn rrc_h(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.HL.as_pair.0);
}

//  ############# L #############

pub fn rl_l(memory: &mut Memory, _value: Void) {
    template_rl!(memory, memory.registers.HL.as_pair.1);
}

pub fn rlc_l(memory: &mut Memory, _value: Void) {
    template_rlc!(memory, memory.registers.HL.as_pair.1);
}

pub fn rr_l(memory: &mut Memory, _value: Void) {
    template_rr!(memory, memory.registers.HL.as_pair.1);
}

pub fn rrc_l(memory: &mut Memory, _value: Void) {
    template_rrc!(memory, memory.registers.HL.as_pair.1);
}

//  ############# HL #############

pub fn rl_hl_addr(memory: &mut Memory, _value: Void) {
    unsafe {
        let hl_value = memory.registers.get_hl() as FarAddress;
        let old_value = memory.read_far_addr(hl_value);
        let old_carry = memory.registers.get_carry_flag();
        let popped_value = get_bit(old_value, max_bit_index(old_value));
        let new_value = assign_bit(old_value << 1, 0, old_carry);

        log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

        memory.write_far_addr(hl_value, new_value);

        memory.registers.set_zero_flag(new_value == 0);
        memory.registers.set_subtraction_flag(false);
        memory.registers.set_half_carry_flag(false);
        memory.registers.set_carry_flag(popped_value);
    }
}

pub fn rlc_hl_addr(memory: &mut Memory, _value: Void) {
        unsafe {
            let hl_value = memory.registers.get_hl() as FarAddress;
            let old_value = memory.read_far_addr(hl_value);
            let popped_value = get_bit(old_value, max_bit_index(old_value));
            // Left shift and put back top bit in the lowest bit
            let new_value = assign_bit(old_value << 1, 0, popped_value);

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            memory.write_far_addr(hl_value, new_value);

            memory.registers.set_zero_flag(new_value == 0);
            memory.registers.set_subtraction_flag(false);
            memory.registers.set_half_carry_flag(false);
            memory.registers.set_carry_flag(popped_value);
        }
}

pub fn rr_hl_addr(memory: &mut Memory, _value: Void) {
        unsafe {
            let hl_value = memory.registers.get_hl() as FarAddress;
            let old_value = memory.read_far_addr(hl_value);
            let old_carry = memory.registers.get_carry_flag();
            let popped_value = get_bit(old_value, 0);
            let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), old_carry);

            log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, old_carry as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            memory.write_far_addr(hl_value, new_value);

            memory.registers.set_zero_flag(new_value == 0);
            memory.registers.set_subtraction_flag(false);
            memory.registers.set_half_carry_flag(false);
            memory.registers.set_carry_flag(popped_value);
        }
}

pub fn rrc_hl_addr(memory: &mut Memory, _value: Void) {
        unsafe {
            let hl_value = memory.registers.get_hl() as FarAddress;
            let old_value = memory.read_far_addr(hl_value);
            let popped_value = get_bit(old_value, 0);
            let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), popped_value);

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            // Left shift and put back top bit in the lowest bit
            memory.write_far_addr(hl_value, new_value);

            memory.registers.set_zero_flag(new_value == 0);
            memory.registers.set_subtraction_flag(false);
            memory.registers.set_half_carry_flag(false);
            memory.registers.set_carry_flag(popped_value);
        }
}