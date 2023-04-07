use crate::cpu::memory::Memory;
use crate::utils::bits::{assign_bit, bit_size, clear_bit, get_bit, max_bit_index, set_bit, swap};
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

macro_rules! template_sla {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let popped_value = get_bit(old_value, max_bit_index(old_value));
            let new_value = old_value << 1;

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_sra {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let top_duplicate = get_bit(old_value, max_bit_index(old_value));
            let popped_value = get_bit(old_value, 0);
            let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), top_duplicate);

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_srl {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let popped_value = get_bit(old_value, 0);
            let new_value = old_value >> 1;

            log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(false);
            $memory.registers.set_carry_flag(popped_value);
        }
    };
}

macro_rules! template_swap {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let new_value = swap(old_value);

            $field = new_value;

            $memory.registers.set_zero_flag(new_value == 0);
        }
    };
}

macro_rules! template_bit {
    ($memory: expr, $field: expr, $index: literal) => {
        unsafe {
            let value = $field;
            let index = $index as usize;

            debug_assert!(index < max_bit_index(value));

            let popped_bit = get_bit(value, index);

            $memory.registers.set_zero_flag(popped_bit);
            $memory.registers.set_subtraction_flag(false);
            $memory.registers.set_half_carry_flag(true);
        }
    };
}

fn template_bit_hl_addr(memory: &mut Memory, index: usize) {
    let value = memory.read_far_addr(memory.registers.get_hl() as FarAddress);

    debug_assert!(index < max_bit_index(value));

    let popped_bit = get_bit(value, index);

    memory.registers.set_zero_flag(popped_bit);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(true);
}

macro_rules! template_res {
    ($field: expr, $index: literal) => {
        unsafe {
            let value = $field;
            let index = $index as usize;

            debug_assert!($index < max_bit_index(value));

            $field = clear_bit(value, index);
        }
    };
}

fn template_res_hl_addr(memory: &mut Memory, index: usize) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let value = memory.read_far_addr(hl_value);

    debug_assert!(index < max_bit_index(value));

    memory.write_far_addr(hl_value, clear_bit(value, index));
}

macro_rules! template_set {
    ($field: expr, $index: literal) => {
        unsafe {
            let value = $field;
            let index = $index as usize;

            debug_assert!($index < max_bit_index(value));

            $field = set_bit(value, index);
        }
    };
}

fn template_set_hl_addr(memory: &mut Memory, index: usize) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let value = memory.read_far_addr(hl_value);

    debug_assert!(index < max_bit_index(value));

    memory.write_far_addr(hl_value, set_bit(value, index));
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
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let old_carry = memory.registers.get_carry_flag();
    let popped_value = get_bit(old_value, max_bit_index(old_value));
    let new_value = assign_bit(old_value << 1, 0, old_carry);

    log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, u8::from(old_carry), new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rlc_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let popped_value = get_bit(old_value, max_bit_index(old_value));
    // Left shift and put back top bit in the lowest bit
    let new_value = assign_bit(old_value << 1, 0, popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rr_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let old_carry = memory.registers.get_carry_flag();
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), old_carry);

    log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, u8::from(old_carry), new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn rrc_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    // Left shift and put back top bit in the lowest bit
    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

//  #############################
//  #           SHIFT           #
//  #############################

//  ############# A #############

pub fn sla_a(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.AF.as_pair.0);
}

pub fn sra_a(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.AF.as_pair.0);
}

pub fn srl_a(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.AF.as_pair.0);
}

//  ############# B #############

pub fn sla_b(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.BC.as_pair.0);
}

pub fn sra_b(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.BC.as_pair.0);
}

pub fn srl_b(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.BC.as_pair.0);
}

//  ############# C #############

pub fn sla_c(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.BC.as_pair.1);
}

pub fn sra_c(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.BC.as_pair.1);
}

pub fn srl_c(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.BC.as_pair.1);
}

//  ############# D #############

pub fn sla_d(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.DE.as_pair.0);
}

pub fn sra_d(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.DE.as_pair.0);
}

pub fn srl_d(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.DE.as_pair.0);
}

//  ############# E #############

pub fn sla_e(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.DE.as_pair.1);
}

pub fn sra_e(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.DE.as_pair.1);
}

pub fn srl_e(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.DE.as_pair.1);
}

//  ############# H #############

pub fn sla_h(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.HL.as_pair.0);
}

pub fn sra_h(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.HL.as_pair.0);
}

pub fn srl_h(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.HL.as_pair.0);
}

//  ############# L #############

pub fn sla_l(memory: &mut Memory, _value: Void) {
    template_sla!(memory, memory.registers.HL.as_pair.1);
}

pub fn sra_l(memory: &mut Memory, _value: Void) {
    template_sra!(memory, memory.registers.HL.as_pair.1);
}

pub fn srl_l(memory: &mut Memory, _value: Void) {
    template_srl!(memory, memory.registers.HL.as_pair.1);
}

//  ############# HL #############

pub fn sla_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let popped_value = get_bit(old_value, max_bit_index(old_value));
    let new_value = old_value << 1;

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn sra_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let top_duplicate = get_bit(old_value, max_bit_index(old_value));
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, max_bit_index(old_value), top_duplicate);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn srl_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);
    let popped_value = get_bit(old_value, 0);
    let new_value = old_value >> 1;

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, u8::from(popped_value), width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

//  #############################
//  #            SWAP           #
//  #############################

pub fn swap_a(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.AF.as_pair.0);
}

pub fn swap_b(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.BC.as_pair.0);
}

pub fn swap_c(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.BC.as_pair.1);
}

pub fn swap_d(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.DE.as_pair.0);
}

pub fn swap_e(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.DE.as_pair.1);
}

pub fn swap_h(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.HL.as_pair.0);
}

pub fn swap_l(memory: &mut Memory, _value: Void) {
    template_swap!(memory, memory.registers.HL.as_pair.1);
}

pub fn swap_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl() as FarAddress;
    let old_value = memory.read_far_addr(hl_value);

    let bit_size_half = bit_size(old_value) / 2;

    let high = old_value >> bit_size_half;
    let low = old_value & ((1 << (bit_size_half + 1)) - 1);

    let new_value = (low << bit_size_half) + high;

    log!("OPERATION", format!("{:#0width$b}=> {:#0width$b}", old_value, new_value, width = bit_size(old_value) + 2));

    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
}

//  #############################
//  #            BIT            #
//  #############################

//  ############# A #############

pub fn bit_0_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 0);
}

pub fn bit_1_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 1);
}

pub fn bit_2_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 2);
}

pub fn bit_3_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 3);
}

pub fn bit_4_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 4);
}

pub fn bit_5_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 5);
}

pub fn bit_6_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 6);
}

pub fn bit_7_a(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.AF.as_pair.0, 7);
}

//  ############# B #############

pub fn bit_0_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 0);
}

pub fn bit_1_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 1);
}

pub fn bit_2_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 2);
}

pub fn bit_3_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 3);
}

pub fn bit_4_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 4);
}

pub fn bit_5_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 5);
}

pub fn bit_6_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 6);
}

pub fn bit_7_b(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.0, 7);
}

//  ############# C #############

pub fn bit_0_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 0);
}

pub fn bit_1_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 1);
}

pub fn bit_2_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 2);
}

pub fn bit_3_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 3);
}

pub fn bit_4_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 4);
}

pub fn bit_5_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 5);
}

pub fn bit_6_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 6);
}

pub fn bit_7_c(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.BC.as_pair.1, 7);
}

//  ############# D #############

pub fn bit_0_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 0);
}

pub fn bit_1_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 1);
}

pub fn bit_2_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 2);
}

pub fn bit_3_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 3);
}

pub fn bit_4_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 4);
}

pub fn bit_5_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 5);
}

pub fn bit_6_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 6);
}

pub fn bit_7_d(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.0, 7);
}

//  ############# E #############

pub fn bit_0_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 0);
}

pub fn bit_1_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 1);
}

pub fn bit_2_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 2);
}

pub fn bit_3_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 3);
}

pub fn bit_4_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 4);
}

pub fn bit_5_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 5);
}

pub fn bit_6_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 6);
}

pub fn bit_7_e(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.DE.as_pair.1, 7);
}

//  ############# H #############

pub fn bit_0_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 0);
}

pub fn bit_1_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 1);
}

pub fn bit_2_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 2);
}

pub fn bit_3_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 3);
}

pub fn bit_4_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 4);
}

pub fn bit_5_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 5);
}

pub fn bit_6_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 6);
}

pub fn bit_7_h(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.0, 7);
}

//  ############# L #############

pub fn bit_0_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 0);
}

pub fn bit_1_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 1);
}

pub fn bit_2_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 2);
}

pub fn bit_3_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 3);
}

pub fn bit_4_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 4);
}

pub fn bit_5_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 5);
}

pub fn bit_6_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 6);
}

pub fn bit_7_l(memory: &mut Memory, _value: Void) {
    template_bit!(memory, memory.registers.HL.as_pair.1, 7);
}

//  ############# HL #############

pub fn bit_0_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 0);
}

pub fn bit_1_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 1);
}

pub fn bit_2_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 2);
}

pub fn bit_3_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 3);
}

pub fn bit_4_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 4);
}

pub fn bit_5_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 5);
}

pub fn bit_6_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 6);
}

pub fn bit_7_hl_addr(memory: &mut Memory, _value: Void) {
    template_bit_hl_addr(memory, 7);
}

//  #############################
//  #            RES            #
//  #############################

//  ############# A #############

pub fn res_0_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 0);
}

pub fn res_1_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 1);
}

pub fn res_2_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 2);
}

pub fn res_3_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 3);
}

pub fn res_4_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 4);
}

pub fn res_5_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 5);
}

pub fn res_6_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 6);
}

pub fn res_7_a(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.AF.as_pair.0, 7);
}

//  ############# B #############

pub fn res_0_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 0);
}

pub fn res_1_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 1);
}

pub fn res_2_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 2);
}

pub fn res_3_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 3);
}

pub fn res_4_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 4);
}

pub fn res_5_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 5);
}

pub fn res_6_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 6);
}

pub fn res_7_b(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.0, 7);
}

//  ############# C #############

pub fn res_0_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 0);
}

pub fn res_1_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 1);
}

pub fn res_2_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 2);
}

pub fn res_3_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 3);
}

pub fn res_4_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 4);
}

pub fn res_5_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 5);
}

pub fn res_6_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 6);
}

pub fn res_7_c(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.BC.as_pair.1, 7);
}

//  ############# D #############

pub fn res_0_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 0);
}

pub fn res_1_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 1);
}

pub fn res_2_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 2);
}

pub fn res_3_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 3);
}

pub fn res_4_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 4);
}

pub fn res_5_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 5);
}

pub fn res_6_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 6);
}

pub fn res_7_d(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.0, 7);
}

//  ############# E #############

pub fn res_0_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 0);
}

pub fn res_1_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 1);
}

pub fn res_2_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 2);
}

pub fn res_3_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 3);
}

pub fn res_4_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 4);
}

pub fn res_5_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 5);
}

pub fn res_6_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 6);
}

pub fn res_7_e(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.DE.as_pair.1, 7);
}

//  ############# H #############

pub fn res_0_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 0);
}

pub fn res_1_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 1);
}

pub fn res_2_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 2);
}

pub fn res_3_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 3);
}

pub fn res_4_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 4);
}

pub fn res_5_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 5);
}

pub fn res_6_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 6);
}

pub fn res_7_h(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.0, 7);
}

//  ############# L #############

pub fn res_0_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 0);
}

pub fn res_1_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 1);
}

pub fn res_2_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 2);
}

pub fn res_3_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 3);
}

pub fn res_4_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 4);
}

pub fn res_5_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 5);
}

pub fn res_6_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 6);
}

pub fn res_7_l(memory: &mut Memory, _value: Void) {
    template_res!(memory.registers.HL.as_pair.1, 7);
}

//  ############# HL #############

pub fn res_0_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 0);
}

pub fn res_1_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 1);
}

pub fn res_2_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 2);
}

pub fn res_3_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 3);
}

pub fn res_4_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 4);
}

pub fn res_5_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 5);
}

pub fn res_6_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 6);
}

pub fn res_7_hl_addr(memory: &mut Memory, _value: Void) {
    template_res_hl_addr(memory, 7);
}

//  #############################
//  #            SET            #
//  #############################

//  ############# A #############

pub fn set_0_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 0);
}

pub fn set_1_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 1);
}

pub fn set_2_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 2);
}

pub fn set_3_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 3);
}

pub fn set_4_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 4);
}

pub fn set_5_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 5);
}

pub fn set_6_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 6);
}

pub fn set_7_a(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.AF.as_pair.0, 7);
}

//  ############# B #############

pub fn set_0_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 0);
}

pub fn set_1_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 1);
}

pub fn set_2_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 2);
}

pub fn set_3_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 3);
}

pub fn set_4_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 4);
}

pub fn set_5_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 5);
}

pub fn set_6_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 6);
}

pub fn set_7_b(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.0, 7);
}

//  ############# C #############

pub fn set_0_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 0);
}

pub fn set_1_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 1);
}

pub fn set_2_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 2);
}

pub fn set_3_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 3);
}

pub fn set_4_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 4);
}

pub fn set_5_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 5);
}

pub fn set_6_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 6);
}

pub fn set_7_c(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.BC.as_pair.1, 7);
}

//  ############# D #############

pub fn set_0_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 0);
}

pub fn set_1_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 1);
}

pub fn set_2_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 2);
}

pub fn set_3_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 3);
}

pub fn set_4_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 4);
}

pub fn set_5_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 5);
}

pub fn set_6_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 6);
}

pub fn set_7_d(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.0, 7);
}

//  ############# E #############

pub fn set_0_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 0);
}

pub fn set_1_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 1);
}

pub fn set_2_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 2);
}

pub fn set_3_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 3);
}

pub fn set_4_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 4);
}

pub fn set_5_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 5);
}

pub fn set_6_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 6);
}

pub fn set_7_e(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.DE.as_pair.1, 7);
}

//  ############# H #############

pub fn set_0_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 0);
}

pub fn set_1_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 1);
}

pub fn set_2_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 2);
}

pub fn set_3_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 3);
}

pub fn set_4_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 4);
}

pub fn set_5_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 5);
}

pub fn set_6_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 6);
}

pub fn set_7_h(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.0, 7);
}

//  ############# L #############

pub fn set_0_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 0);
}

pub fn set_1_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 1);
}

pub fn set_2_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 2);
}

pub fn set_3_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 3);
}

pub fn set_4_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 4);
}

pub fn set_5_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 5);
}

pub fn set_6_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 6);
}

pub fn set_7_l(memory: &mut Memory, _value: Void) {
    template_set!(memory.registers.HL.as_pair.1, 7);
}

//  ############# HL #############

pub fn set_0_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 0);
}

pub fn set_1_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 1);
}

pub fn set_2_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 2);
}

pub fn set_3_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 3);
}

pub fn set_4_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 4);
}

pub fn set_5_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 5);
}

pub fn set_6_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 6);
}

pub fn set_7_hl_addr(memory: &mut Memory, _value: Void) {
    template_set_hl_addr(memory, 7);
}
