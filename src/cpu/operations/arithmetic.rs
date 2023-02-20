use crate::cpu::memory::Memory;
use crate::utils::bits::{check_half_carry_add, check_half_carry_wide_add, check_half_carry_sub, bit_size};
use crate::utils::log::log;
use crate::utils::types::{Value, Void};

//  #############################
//  #         Template          #
//  #############################

macro_rules! template_inc_value {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value: Value = $field;
            let new_value: Value = old_value + 1;
            $field = new_value;

            // Operation flags
            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 3.
            $memory.registers.set_half_carry_flag(check_half_carry_add(old_value, 1));
        }
    };
}

macro_rules! template_dec_value {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let new_value = old_value - 1;
            $field = new_value;

            // Operation flags
            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(true);
            // H => Set if borrow from bit 4.
            $memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, 1));
        }
    };
}

macro_rules! template_inc_wide {
    ($field: expr) => {
        unsafe { $field += 1; }
    };
}

macro_rules! template_dec_wide {
    ($field: expr) => {
        unsafe { $field -= 1; }
    };
}

macro_rules! template_add_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_add(value);

            $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 3.
            $memory.registers.set_half_carry_flag(check_half_carry_add(old_value, value));
            // C => Set if overflow from bit 15.
            $memory.registers.set_carry_flag(has_overflown);
        }
    };
}

macro_rules! template_add_hl {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_hl();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_add(value);

            $memory.registers.set_hl(result);

            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 11.
            $memory.registers.set_half_carry_flag(check_half_carry_wide_add(old_value, value));
            // C => Set if overflow from bit 15.
            $memory.registers.set_carry_flag(has_overflown);
        }
    };
}

macro_rules! template_sub_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_sub(value);

            $memory.registers.set_a(result);

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
//  #         Increment         #
//  #############################

//  ########## 8-bits ###########

pub fn inc_a(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.AF.as_pair.0);
}

pub fn inc_b(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.BC.as_pair.0);
}

pub fn inc_c(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.BC.as_pair.1);
}

pub fn inc_d(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.DE.as_pair.0);
}

pub fn inc_e(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.DE.as_pair.1);
}

pub fn inc_h(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.HL.as_pair.0);
}

pub fn inc_l(memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.HL.as_pair.1);
}

pub fn inc_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);
    let new_value = read_value + 1;
    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(check_half_carry_add(read_value, 1));
}

//  ######### 16-bits ###########

pub fn inc_bc(memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.BC.as_wide);
}

pub fn inc_de(memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.DE.as_wide);
}

pub fn inc_hl(memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.HL.as_wide);
}

pub fn inc_sp(memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.SP);
}

//  #############################
//  #         Increment         #
//  #############################

//  ########## 8-bits ###########

pub fn dec_a(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.AF.as_pair.0);
}

pub fn dec_b(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.BC.as_pair.0);
}

pub fn dec_c(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.BC.as_pair.1);
}

pub fn dec_d(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.DE.as_pair.0);
}

pub fn dec_e(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.DE.as_pair.1);
}

pub fn dec_h(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.HL.as_pair.0);
}

pub fn dec_l(memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.HL.as_pair.1);
}

pub fn dec_hl_addr(memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);
    let new_value = read_value - 1;
    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    memory.registers.set_half_carry_flag(check_half_carry_sub(read_value, 1));
}

//  ######### 16-bits ###########

pub fn dec_bc(memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.BC.as_wide);
}

pub fn dec_de(memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.DE.as_wide);
}

pub fn dec_hl(memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.HL.as_wide);
}

pub fn dec_sp(memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.SP);
}

//  #############################
//  #         Addition          #
//  #############################

//  ########## 8-bits ###########

pub fn add_a_d8(memory: &mut Memory, value: Value) {
    template_add_a!(memory, value);
}

pub fn add_a_a(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn add_a_b(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn add_a_c(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn add_a_d(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn add_a_e(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn add_a_h(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn add_a_l(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn add_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  ######### 16-bits ###########

pub fn add_hl_bc(memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.BC.as_wide);
}

pub fn add_hl_de(memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.DE.as_wide);
}

pub fn add_hl_hl(memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.HL.as_wide);
}

pub fn add_hl_sp(memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.SP);
}

//  #############################
//  #        Subtraction        #
//  #############################

pub fn sub_a_a(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn sub_a_b(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn sub_a_c(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn sub_a_d(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn sub_a_e(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn sub_a_h(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn sub_a_l(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn sub_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

//  #############################
//  #    Addition with Carry    #
//  #############################

pub fn adc_a_d8(memory: &mut Memory, value: Value) {
    template_add_a!(memory, value + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_a(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.AF.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_b(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_c(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_d(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_e(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_h(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_l(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.read_far_addr(memory.registers.get_hl()) + (memory.registers.get_carry_flag() as Value));
}

//  #############################
//  #  Subtraction with Carry   #
//  #############################

pub fn sbc_a_a(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.AF.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_b(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_c(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_d(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_e(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_h(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_l(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_hl_addr(memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.read_far_addr(memory.registers.get_hl()) + (memory.registers.get_carry_flag() as Value));
}

//  #############################
//  #           Misc            #
//  #############################

/// Complement accumulator
pub fn cpl(memory: &mut Memory, _value: Void) {
    memory.registers.set_a(!memory.registers.get_a());

    memory.registers.set_subtraction_flag(true);
    memory.registers.set_half_carry_flag(true);
}

/// Complement carry flag
pub fn ccf(memory: &mut Memory, _value: Void) {
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(!memory.registers.get_carry_flag());
}

// Set carry flag
pub fn scf(memory: &mut Memory, _value: Void) {
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(true);
}

// TODO: Check
/// Decimal Adjust Accumulator to get a correct BCD representation after an arithmetic instruction.
pub fn daa(memory: &mut Memory, _value: Void) {
    // Documentation:
    // http://z80-heaven.wikidot.com/instructions-set:daa
    // https://fr.wikibooks.org/wiki/Programmation_Assembleur_Z80/Jeu_d_instructions#DAA

    let old_value = memory.registers.get_a();
    let old_half_carry_flag = memory.registers.get_half_carry_flag();
    let old_carry_flag = memory.registers.get_carry_flag();
    let mut new_value = old_value;

    if old_half_carry_flag || (old_value & 0b1111) > 9  {
        new_value = new_value.wrapping_add(0x06)
    }

    if old_carry_flag || ((old_value >> 4) & 0b1111) > 9 {
        new_value = new_value.wrapping_add(0x60);
        memory.registers.set_carry_flag(true);
    } else {
        memory.registers.set_carry_flag(false);
    }

    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_half_carry_flag(false);

    log!("OPERATION", format!("{} ({:#0width$b}) + C={} + H={} => {} ({:#0width$b}) + C={} + Z={}", old_value, old_value, old_carry_flag as u8, old_half_carry_flag as u8, new_value, new_value, memory.registers.get_carry_flag() as u8, memory.registers.get_zero_flag() as u8, width = bit_size(old_value) + 2));
}