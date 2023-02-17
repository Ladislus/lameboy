use sdl2::libc::CTRL_CMD_DELFAMILY;

use crate::cpu::instruction::{GenericInstruction, Instruction, WideValueInstruction, ValueInstruction, FarAddressInstruction, OffsetInstruction, VoidInstruction};
use crate::cpu::memory::Memory;
use crate::cpu::template::{template_add_a, template_add_hl, template_and_a, template_cp_a, template_dec_value, template_dec_wide, template_inc_value, template_inc_wide, template_ld, template_or_a, template_sub_a, template_xor_a};
use crate::utils::bits::{assign_bit, bit_size, check_half_carry_add, check_half_carry_sub, check_half_carry_wide_add, get_bit, max_bit_index};
use crate::utils::log::log;
use crate::utils::types::{FarAddress, AddressOffset, Value, Void, WideValue};

pub fn unimplemented<T>(instr: &Instruction<T>, _memory: &mut Memory, _value: T) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {}

pub fn ld_bc_d16(_instr: &WideValueInstruction, memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.BC.as_wide, value);
}

pub fn ld_bc_addr_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_bc(), memory.registers.get_a());
}

pub fn inc_bc(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.BC.as_wide);
}

pub fn inc_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.BC.as_pair.0);
}

pub fn dec_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.BC.as_pair.0);
}

pub fn ld_b_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.BC.as_pair.0, value);
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

pub fn ld_a16_addr_sp(_instr: &FarAddressInstruction, memory: &mut Memory, value: FarAddress) {
    memory.write_wide_far_addr(value, memory.registers.SP);
}

pub fn add_hl_bc(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.BC.as_wide);
}

pub fn ld_a_bc_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_bc()));
}

pub fn dec_bc(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.BC.as_wide);
}

pub fn inc_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.BC.as_pair.1);
}

pub fn dec_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.BC.as_pair.1);
}

pub fn ld_c_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.BC.as_pair.1, value);
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

pub fn stop(_instr: &Instruction<u8>, _memory: &mut Memory, _value: u8) {
    // https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/#STOP
    todo!("Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.")
}

pub fn ld_de_d16(_instr: &WideValueInstruction, memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.DE.as_wide, value);
}

pub fn ld_de_addr_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_de(), memory.registers.get_a());
}

pub fn inc_de(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.DE.as_wide);
}

pub fn inc_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.DE.as_pair.0);
}

pub fn dec_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.DE.as_pair.0);
}

pub fn ld_d_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.DE.as_pair.0, value);
}

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

pub fn jr_r8(_instr: &OffsetInstruction, memory: &mut Memory, value: AddressOffset) {
    // Relative Jump to address n16. The address is encoded as a signed 8-bit offset from the address immediately following the JR instruction, so the target address n16 must be between -128 and 127 bytes away.
    // For example:
    //     JR Label  ; no-op; encoded offset of 0
    // Label:
    //     JR Label  ; infinite loop; encoded offset of -2
    // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
    memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
}

pub fn add_hl_de(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.DE.as_wide);
}

pub fn ld_a_de_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_de()));
}

pub fn dec_de(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.DE.as_wide);
}

pub fn inc_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.DE.as_pair.1);
}

pub fn dec_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.DE.as_pair.1);
}

pub fn ld_e_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.DE.as_pair.1, value);
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

pub fn jr_nz_r8(_instr: &OffsetInstruction, memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_zero_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn ld_hl_d16(_instr: &WideValueInstruction, memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.HL.as_wide, value);
}

pub fn ld_hli_addr_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    // This is sometimes written as ‘LD (HLI),A’, or ‘LDI (HL),A’.
    let hl_value = memory.registers.get_hl();
    memory.write_far_addr(hl_value, memory.registers.get_a());
    memory.registers.set_hl(hl_value + 1);
}

pub fn inc_hl(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.HL.as_wide);
}

pub fn inc_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.HL.as_pair.0);
}

pub fn dec_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.HL.as_pair.0);

}

pub fn ld_h_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.HL.as_pair.0, value);
}

// TODO: Check
pub fn daa(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
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

pub fn jr_z_r8(_instr: &OffsetInstruction, memory: &mut Memory, value: AddressOffset) {
    if memory.registers.get_zero_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn add_hl_hl(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.HL.as_wide);
}

pub fn ld_a_hli_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);

    memory.registers.set_hl(hl_value + 1);
    memory.registers.set_a(read_value);
}

pub fn dec_hl(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.HL.as_wide);
}

pub fn inc_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.HL.as_pair.1);
}

pub fn dec_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.HL.as_pair.1);
}

pub fn ld_l_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.HL.as_pair.1, value);
}

pub fn cpl(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.registers.set_a(!memory.registers.get_a());

    memory.registers.set_subtraction_flag(true);
    memory.registers.set_half_carry_flag(true);
}

pub fn jr_nc_r8(_instr: &OffsetInstruction, memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_carry_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn ld_sp_d16(_instr: &WideValueInstruction, memory: &mut Memory, value: WideValue) {
    template_ld!(memory.registers.SP, value);
}

pub fn ld_hld_addr_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    // This is sometimes written as ‘LD (HLD),A’, or ‘LDD (HL),A’.
    let hl_value = memory.registers.get_hl();
    memory.write_far_addr(hl_value, memory.registers.get_a());
    memory.registers.set_hl(hl_value - 1);
}

pub fn inc_sp(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_wide!(memory.registers.SP);
}

pub fn inc_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);
    let new_value = read_value + 1;
    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(check_half_carry_add(read_value, 1));
}

pub fn dec_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);
    let new_value = read_value - 1;
    memory.write_far_addr(hl_value, new_value);

    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    memory.registers.set_half_carry_flag(check_half_carry_sub(read_value, 1));
}

pub fn ld_hl_addr_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    memory.write_far_addr(memory.registers.get_hl(), value);
}

pub fn scf(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(true);
}

pub fn jr_c_r8(_instr: &OffsetInstruction, memory: &mut Memory, value: AddressOffset) {
    if memory.registers.get_carry_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn add_hl_sp(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_hl!(memory, memory.registers.SP);
}

pub fn ld_a_hld_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    let hl_value = memory.registers.get_hl();
    let read_value = memory.read_far_addr(hl_value);

    memory.registers.set_hl(hl_value - 1);
    memory.registers.set_a(read_value);
}

pub fn dec_sp(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_wide!(memory.registers.SP);
}

pub fn inc_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_inc_value!(memory, memory.registers.AF.as_pair.0);
}

pub fn dec_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_dec_value!(memory, memory.registers.AF.as_pair.0);
}

pub fn ld_a_d8(_instr: &ValueInstruction, memory: &mut Memory, value: Value) {
    template_ld!(memory.registers.AF.as_pair.0, value);
}

pub fn ccf(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(!memory.registers.get_carry_flag());
}

pub fn ld_b_b(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.BC.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_b_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_b_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_b_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_b_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_b_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_b_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_b_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_c_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_c_c(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.BC.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_c_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_c_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_c_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_c_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_c_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_c_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.BC.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_d_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_d_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_d_d(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.DE.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_d_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_d_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_d_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_d_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_d_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_e_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_e_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_e_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_e_e(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.DE.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_e_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_e_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_e_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_e_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.DE.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_h_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_h_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_h_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_h_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_h_h(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.HL.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_h_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_h_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_h_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn ld_l_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.BC.as_pair.0);
}

pub fn ld_l_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.BC.as_pair.1);
}

pub fn ld_l_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.DE.as_pair.0);
}

pub fn ld_l_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.DE.as_pair.1);
}

pub fn ld_l_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.HL.as_pair.0);
}

pub fn ld_l_l(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.HL.as_pair.1, memory.registers.HL.as_pair.1);
}

pub fn ld_l_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_l_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.HL.as_pair.1, memory.registers.AF.as_pair.0);
}

pub fn ld_hl_addr_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_b());
}

pub fn ld_hl_addr_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_c());
}

pub fn ld_hl_addr_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_d());
}

pub fn ld_hl_addr_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_e());
}

pub fn ld_hl_addr_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_h());
}

pub fn ld_hl_addr_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_l());
}

pub fn halt(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/#HALT
    todo!("Enter CPU low-power consumption mode until an interrupt occurs. The exact behavior of this instruction depends on the state of the IME flag.")
}

pub fn ld_hl_addr_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_hl(), memory.registers.get_a());
}

pub fn ld_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.BC.as_pair.0);
}

pub fn ld_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.BC.as_pair.1);
}

pub fn ld_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.DE.as_pair.0);
}

pub fn ld_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.DE.as_pair.1);
}

pub fn ld_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.HL.as_pair.0);
}

pub fn ld_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.registers.HL.as_pair.1);
}

pub fn ld_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_ld!(memory.registers.AF.as_pair.0, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn ld_a_a(_instr: &VoidInstruction, _memory: &mut Memory, _value: Void) {
    // Equivalent to a NOP
    // https://retrocomputing.stackexchange.com/questions/19632/what-could-be-the-reason-an-ld-b-b-instruction-was-used-in-this-busy-loop
    // template_ld!(memory.registers.AF.as_pair.0, memory.registers.AF.as_pair.0);
}

pub fn add_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn add_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn add_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn add_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn add_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn add_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn add_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn add_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn adc_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.BC.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.DE.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.HL.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.read_far_addr(memory.registers.get_hl()) + (memory.registers.get_carry_flag() as Value));
}

pub fn adc_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_add_a!(memory, memory.registers.AF.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sub_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.0);
}

pub fn sub_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.1);
}

pub fn sub_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.0);
}

pub fn sub_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.1);
}

pub fn sub_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.0);
}

pub fn sub_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.1);
}

pub fn sub_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.read_far_addr(memory.registers.get_hl()));
}

pub fn sub_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn sbc_a_b(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_c(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.BC.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_d(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_e(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.DE.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_h(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.0 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_l(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.HL.as_pair.1 + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_hl_addr(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.read_far_addr(memory.registers.get_hl()) + (memory.registers.get_carry_flag() as Value));
}

pub fn sbc_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_sub_a!(memory, memory.registers.AF.as_pair.0 + (memory.registers.get_carry_flag() as Value));
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

pub fn and_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_and_a!(memory, memory.registers.AF.as_pair.0);
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

pub fn xor_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_xor_a!(memory, memory.registers.AF.as_pair.0);
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

pub fn or_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_or_a!(memory, memory.registers.AF.as_pair.0);
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

pub fn cp_a_a(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    template_cp_a!(memory, memory.registers.AF.as_pair.0);
}

pub fn ret_nz(_instr: &VoidInstruction, memory: &mut Memory, _value: Void) {
    if !memory.registers.get_zero_flag() {
        // TODO: Check
        debug_assert!(memory.stack.len() > 0);
        memory.registers.SP = memory.stack.pop().unwrap();
    }
}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [GenericInstruction; 193] = [
    GenericInstruction::VOID(  Instruction { opcode: 0x00, disassembly: "NOP"         , byte_size: 1, clock_tick: 4 , function: noop }),
    GenericInstruction::DATA16(Instruction { opcode: 0x01, disassembly: "LD BC, d16"  , byte_size: 3, clock_tick: 12, function: ld_bc_d16 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x02, disassembly: "LD (BC), A"  , byte_size: 1, clock_tick: 8 , function: ld_bc_addr_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x03, disassembly: "INC BC"      , byte_size: 1, clock_tick: 8 , function: inc_bc }),
    GenericInstruction::VOID(  Instruction { opcode: 0x04, disassembly: "INC B"       , byte_size: 1, clock_tick: 4 , function: inc_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x05, disassembly: "DEC B"       , byte_size: 1, clock_tick: 4 , function: dec_b }),
    GenericInstruction::DATA8( Instruction { opcode: 0x06, disassembly: "LD B, d8"    , byte_size: 2, clock_tick: 8 , function: ld_b_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x07, disassembly: "RLCA"        , byte_size: 1, clock_tick: 4 , function: rlca }),
    GenericInstruction::ADDR16(Instruction { opcode: 0x08, disassembly: "LD (a16), SP", byte_size: 3, clock_tick: 20, function: ld_a16_addr_sp }),
    GenericInstruction::VOID(  Instruction { opcode: 0x09, disassembly: "AD HL, BC"   , byte_size: 1, clock_tick: 8 , function: add_hl_bc }),
    GenericInstruction::VOID(  Instruction { opcode: 0x0A, disassembly: "LD A, (BC)"  , byte_size: 1, clock_tick: 8 , function: ld_a_bc_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x0B, disassembly: "DEC BC"      , byte_size: 1, clock_tick: 8 , function: dec_bc }),
    GenericInstruction::VOID(  Instruction { opcode: 0x0C, disassembly: "INC C"       , byte_size: 1, clock_tick: 4 , function: inc_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x0D, disassembly: "DEC C"       , byte_size: 1, clock_tick: 4 , function: dec_c }),
    GenericInstruction::DATA8( Instruction { opcode: 0x0E, disassembly: "LD C, d8"    , byte_size: 2, clock_tick: 8 , function: ld_c_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x0F, disassembly: "RRCA"        , byte_size: 1, clock_tick: 4 , function: rrca }),
    GenericInstruction::DATA8( Instruction { opcode: 0x10, disassembly: "STOP d8"     , byte_size: 2, clock_tick: 4 , function: stop }),
    GenericInstruction::DATA16(Instruction { opcode: 0x11, disassembly: "LD DE, d16"  , byte_size: 3, clock_tick: 12, function: ld_de_d16 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x12, disassembly: "LD (DE), A"  , byte_size: 1, clock_tick: 8 , function: ld_de_addr_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x13, disassembly: "INC DE"      , byte_size: 1, clock_tick: 8 , function: inc_de }),
    GenericInstruction::VOID(  Instruction { opcode: 0x14, disassembly: "INC D"       , byte_size: 1, clock_tick: 4 , function: inc_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x15, disassembly: "DEC D"       , byte_size: 1, clock_tick: 4 , function: dec_d }),
    GenericInstruction::DATA8( Instruction { opcode: 0x16, disassembly: "LD D, d8"    , byte_size: 2, clock_tick: 8 , function: ld_d_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x17, disassembly: "RLA"         , byte_size: 1, clock_tick: 4 , function: rla }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x18, disassembly: "JR r8"       , byte_size: 2, clock_tick: 12, function: jr_r8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x19, disassembly: "ADD HL, DE"  , byte_size: 1, clock_tick: 8 , function: add_hl_de }),
    GenericInstruction::VOID(  Instruction { opcode: 0x1A, disassembly: "LD A, (DE)"  , byte_size: 1, clock_tick: 8 , function: ld_a_de_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x1B, disassembly: "DEC DE"      , byte_size: 1, clock_tick: 8 , function: dec_de }),
    GenericInstruction::VOID(  Instruction { opcode: 0x1C, disassembly: "INC E"       , byte_size: 1, clock_tick: 4 , function: inc_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x1D, disassembly: "DEC E"       , byte_size: 1, clock_tick: 4 , function: dec_e }),
    GenericInstruction::DATA8( Instruction { opcode: 0x1E, disassembly: "LD E, d8"    , byte_size: 2, clock_tick: 8 , function: ld_e_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x1F, disassembly: "RRA"         , byte_size: 1, clock_tick: 4 , function: rra }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x20, disassembly: "JR NZ, r8"   , byte_size: 1, clock_tick: 8 , function: jr_nz_r8 }),
    GenericInstruction::DATA16(Instruction { opcode: 0x21, disassembly: "LD HL, d16"  , byte_size: 3, clock_tick: 12, function: ld_hl_d16 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x22, disassembly: "LD (HL+), A" , byte_size: 1, clock_tick: 8 , function: ld_hli_addr_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x23, disassembly: "INC HL"      , byte_size: 1, clock_tick: 8 , function: inc_hl }),
    GenericInstruction::VOID(  Instruction { opcode: 0x24, disassembly: "INC H"       , byte_size: 1, clock_tick: 4 , function: inc_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x25, disassembly: "DEC H"       , byte_size: 1, clock_tick: 4 , function: dec_h }),
    GenericInstruction::DATA8( Instruction { opcode: 0x26, disassembly: "LD H, d8"    , byte_size: 2, clock_tick: 8 , function: ld_h_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x27, disassembly: "DAA"         , byte_size: 1, clock_tick: 4 , function: daa }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x28, disassembly: "JR Z, r8"    , byte_size: 2, clock_tick: 8 , function: jr_z_r8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x29, disassembly: "ADD HL, HL"  , byte_size: 1, clock_tick: 8 , function: add_hl_hl }),
    GenericInstruction::VOID(  Instruction { opcode: 0x2A, disassembly: "LD A, (HL+)" , byte_size: 1, clock_tick: 8 , function: ld_a_hli_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x2B, disassembly: "DEC HL"      , byte_size: 1, clock_tick: 8 , function: dec_hl }),
    GenericInstruction::VOID(  Instruction { opcode: 0x2C, disassembly: "INC L"       , byte_size: 1, clock_tick: 4 , function: inc_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x2D, disassembly: "DEC L"       , byte_size: 1, clock_tick: 4 , function: dec_l }),
    GenericInstruction::DATA8( Instruction { opcode: 0x2E, disassembly: "LD L, d8"    , byte_size: 2, clock_tick: 8 , function: ld_l_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x2F, disassembly: "CPL"         , byte_size: 1, clock_tick: 4 , function: cpl }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x30, disassembly: "JR NC, r8"   , byte_size: 2, clock_tick: 8 , function: jr_nc_r8 }),
    GenericInstruction::DATA16(Instruction { opcode: 0x31, disassembly: "LD SP, d16"  , byte_size: 3, clock_tick: 12, function: ld_sp_d16 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x32, disassembly: "LD (HL-), A" , byte_size: 1, clock_tick: 8 , function: ld_hld_addr_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x33, disassembly: "INC SP"      , byte_size: 1, clock_tick: 8 , function: inc_sp }),
    GenericInstruction::VOID(  Instruction { opcode: 0x34, disassembly: "INC (HL)"    , byte_size: 1, clock_tick: 12, function: inc_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x35, disassembly: "DEC (HL)"    , byte_size: 1, clock_tick: 12, function: dec_hl_addr }),
    GenericInstruction::DATA8( Instruction { opcode: 0x36, disassembly: "LD (HL), d8" , byte_size: 2, clock_tick: 12, function: ld_hl_addr_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x37, disassembly: "SCF"         , byte_size: 1, clock_tick: 4 , function: scf }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x38, disassembly: "JR C, r8"    , byte_size: 2, clock_tick: 8 , function: jr_c_r8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x39, disassembly: "ADD HL, SP"  , byte_size: 1, clock_tick: 8 , function: add_hl_sp }),
    GenericInstruction::VOID(  Instruction { opcode: 0x3A, disassembly: "LD A, (HL-)" , byte_size: 1, clock_tick: 8 , function: ld_a_hld_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x3B, disassembly: "DEC SP"      , byte_size: 1, clock_tick: 8 , function: dec_sp }),
    GenericInstruction::VOID(  Instruction { opcode: 0x3C, disassembly: "INC A"       , byte_size: 1, clock_tick: 4 , function: inc_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x3D, disassembly: "DEC A"       , byte_size: 1, clock_tick: 4 , function: dec_a }),
    GenericInstruction::DATA8( Instruction { opcode: 0x3E, disassembly: "LD A, d8"    , byte_size: 2, clock_tick: 8 , function: ld_a_d8 }),
    GenericInstruction::VOID(  Instruction { opcode: 0x3F, disassembly: "CCF"         , byte_size: 1, clock_tick: 4 , function: ccf }),
    GenericInstruction::VOID(  Instruction { opcode: 0x40, disassembly: "LD B, B"     , byte_size: 1, clock_tick: 4 , function: ld_b_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x41, disassembly: "LD B, C"     , byte_size: 1, clock_tick: 4 , function: ld_b_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x42, disassembly: "LD B, D"     , byte_size: 1, clock_tick: 4 , function: ld_b_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x43, disassembly: "LD B, E"     , byte_size: 1, clock_tick: 4 , function: ld_b_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x44, disassembly: "LD B, H"     , byte_size: 1, clock_tick: 4 , function: ld_b_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x45, disassembly: "LD B, L"     , byte_size: 1, clock_tick: 4 , function: ld_b_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x46, disassembly: "LD B, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_b_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x47, disassembly: "LD B, A"     , byte_size: 1, clock_tick: 4 , function: ld_b_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x48, disassembly: "LD C, B"     , byte_size: 1, clock_tick: 4 , function: ld_c_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x49, disassembly: "LD C, C"     , byte_size: 1, clock_tick: 4 , function: ld_c_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4A, disassembly: "LD C, D"     , byte_size: 1, clock_tick: 4 , function: ld_c_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4B, disassembly: "LD C, E"     , byte_size: 1, clock_tick: 4 , function: ld_c_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4C, disassembly: "LD C, H"     , byte_size: 1, clock_tick: 4 , function: ld_c_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4D, disassembly: "LD C, L"     , byte_size: 1, clock_tick: 4 , function: ld_c_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4E, disassembly: "LD C, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_c_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x4F, disassembly: "LD C, A"     , byte_size: 1, clock_tick: 4 , function: ld_c_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x50, disassembly: "LD D, B"     , byte_size: 1, clock_tick: 4 , function: ld_d_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x51, disassembly: "LD D, C"     , byte_size: 1, clock_tick: 4 , function: ld_d_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x52, disassembly: "LD D, D"     , byte_size: 1, clock_tick: 4 , function: ld_d_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x53, disassembly: "LD D, E"     , byte_size: 1, clock_tick: 4 , function: ld_d_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x54, disassembly: "LD D, H"     , byte_size: 1, clock_tick: 4 , function: ld_d_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x55, disassembly: "LD D, L"     , byte_size: 1, clock_tick: 4 , function: ld_d_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x56, disassembly: "LD D, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_d_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x57, disassembly: "LD D, A"     , byte_size: 1, clock_tick: 4 , function: ld_d_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x58, disassembly: "LD E, B"     , byte_size: 1, clock_tick: 4 , function: ld_e_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x59, disassembly: "LD E, C"     , byte_size: 1, clock_tick: 4 , function: ld_e_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5A, disassembly: "LD E, D"     , byte_size: 1, clock_tick: 4 , function: ld_e_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5B, disassembly: "LD E, E"     , byte_size: 1, clock_tick: 4 , function: ld_e_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5C, disassembly: "LD E, H"     , byte_size: 1, clock_tick: 4 , function: ld_e_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5D, disassembly: "LD E, L"     , byte_size: 1, clock_tick: 4 , function: ld_e_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5E, disassembly: "LD E, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_e_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x5F, disassembly: "LD E, A"     , byte_size: 1, clock_tick: 4 , function: ld_e_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x60, disassembly: "LD H, B"     , byte_size: 1, clock_tick: 4 , function: ld_h_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x61, disassembly: "LD H, C"     , byte_size: 1, clock_tick: 4 , function: ld_h_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x62, disassembly: "LD H, D"     , byte_size: 1, clock_tick: 4 , function: ld_h_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x63, disassembly: "LD H, E"     , byte_size: 1, clock_tick: 4 , function: ld_h_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x64, disassembly: "LD H, H"     , byte_size: 1, clock_tick: 4 , function: ld_h_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x65, disassembly: "LD H, L"     , byte_size: 1, clock_tick: 4 , function: ld_h_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x66, disassembly: "LD H, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_h_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x67, disassembly: "LD H, A"     , byte_size: 1, clock_tick: 4 , function: ld_h_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x68, disassembly: "LD L, B"     , byte_size: 1, clock_tick: 4 , function: ld_l_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x69, disassembly: "LD L, C"     , byte_size: 1, clock_tick: 4 , function: ld_l_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6A, disassembly: "LD L, D"     , byte_size: 1, clock_tick: 4 , function: ld_l_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6B, disassembly: "LD L, E"     , byte_size: 1, clock_tick: 4 , function: ld_l_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6C, disassembly: "LD L, H"     , byte_size: 1, clock_tick: 4 , function: ld_l_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6D, disassembly: "LD L, L"     , byte_size: 1, clock_tick: 4 , function: ld_l_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6E, disassembly: "LD L, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_l_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x6F, disassembly: "LD L, A"     , byte_size: 1, clock_tick: 4 , function: ld_l_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x70, disassembly: "LD (HL), B"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x71, disassembly: "LD (HL), C"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x72, disassembly: "LD (HL), D"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x73, disassembly: "LD (HL), E"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x74, disassembly: "LD (HL), H"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x75, disassembly: "LD (HL), L"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x76, disassembly: "HALT"        , byte_size: 1, clock_tick: 4 , function: halt }),
    GenericInstruction::VOID(  Instruction { opcode: 0x77, disassembly: "LD (HL), A"  , byte_size: 1, clock_tick: 8 , function: ld_hl_addr_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x78, disassembly: "LD A, B"     , byte_size: 1, clock_tick: 4 , function: ld_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x79, disassembly: "LD A, C"     , byte_size: 1, clock_tick: 4 , function: ld_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7A, disassembly: "LD A, D"     , byte_size: 1, clock_tick: 4 , function: ld_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7B, disassembly: "LD A, E"     , byte_size: 1, clock_tick: 4 , function: ld_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7C, disassembly: "LD A, H"     , byte_size: 1, clock_tick: 4 , function: ld_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7D, disassembly: "LD A, L"     , byte_size: 1, clock_tick: 4 , function: ld_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7E, disassembly: "LD A, (HL)"  , byte_size: 1, clock_tick: 8 , function: ld_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x7F, disassembly: "LD A, A"     , byte_size: 1, clock_tick: 4 , function: ld_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x80, disassembly: "ADD A, B"    , byte_size: 1, clock_tick: 4 , function: add_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x81, disassembly: "ADD A, C"    , byte_size: 1, clock_tick: 4 , function: add_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x82, disassembly: "ADD A, D"    , byte_size: 1, clock_tick: 4 , function: add_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x83, disassembly: "ADD A, E"    , byte_size: 1, clock_tick: 4 , function: add_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x84, disassembly: "ADD A, H"    , byte_size: 1, clock_tick: 4 , function: add_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x85, disassembly: "ADD A, L"    , byte_size: 1, clock_tick: 4 , function: add_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x86, disassembly: "ADD A, (HL)" , byte_size: 1, clock_tick: 8 , function: add_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x87, disassembly: "ADD A, A"    , byte_size: 1, clock_tick: 4 , function: add_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x88, disassembly: "ADC A, B"    , byte_size: 1, clock_tick: 4 , function: adc_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x89, disassembly: "ADC A, C"    , byte_size: 1, clock_tick: 4 , function: adc_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8A, disassembly: "ADC A, D"    , byte_size: 1, clock_tick: 4 , function: adc_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8B, disassembly: "ADC A, E"    , byte_size: 1, clock_tick: 4 , function: adc_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8C, disassembly: "ADC A, H"    , byte_size: 1, clock_tick: 4 , function: adc_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8D, disassembly: "ADC A, L"    , byte_size: 1, clock_tick: 4 , function: adc_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8E, disassembly: "ADC A, (HL)" , byte_size: 1, clock_tick: 8 , function: adc_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x8F, disassembly: "ADC A, A"    , byte_size: 1, clock_tick: 4 , function: adc_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x90, disassembly: "SUB A, B"    , byte_size: 1, clock_tick: 4 , function: sub_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x91, disassembly: "SUB A, C"    , byte_size: 1, clock_tick: 4 , function: sub_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x92, disassembly: "SUB A, D"    , byte_size: 1, clock_tick: 4 , function: sub_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x93, disassembly: "SUB A, E"    , byte_size: 1, clock_tick: 4 , function: sub_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x94, disassembly: "SUB A, H"    , byte_size: 1, clock_tick: 4 , function: sub_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x95, disassembly: "SUB A, L"    , byte_size: 1, clock_tick: 4 , function: sub_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x96, disassembly: "SUB A, (HL)" , byte_size: 1, clock_tick: 8 , function: sub_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x97, disassembly: "SUB A, A"    , byte_size: 1, clock_tick: 4 , function: sub_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0x98, disassembly: "SBC A, B"    , byte_size: 1, clock_tick: 4 , function: sbc_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0x99, disassembly: "SBC A, C"    , byte_size: 1, clock_tick: 4 , function: sbc_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9A, disassembly: "SBC A, D"    , byte_size: 1, clock_tick: 4 , function: sbc_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9B, disassembly: "SBC A, E"    , byte_size: 1, clock_tick: 4 , function: sbc_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9C, disassembly: "SBC A, H"    , byte_size: 1, clock_tick: 4 , function: sbc_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9D, disassembly: "SBC A, L"    , byte_size: 1, clock_tick: 4 , function: sbc_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9E, disassembly: "SBC A, (HL)" , byte_size: 1, clock_tick: 8 , function: sbc_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0x9F, disassembly: "SBC A, A"    , byte_size: 1, clock_tick: 4 , function: sbc_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA0, disassembly: "AND A, B"    , byte_size: 1, clock_tick: 4 , function: and_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA1, disassembly: "AND A, C"    , byte_size: 1, clock_tick: 4 , function: and_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA2, disassembly: "AND A, D"    , byte_size: 1, clock_tick: 4 , function: and_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA3, disassembly: "AND A, E"    , byte_size: 1, clock_tick: 4 , function: and_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA4, disassembly: "AND A, H"    , byte_size: 1, clock_tick: 4 , function: and_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA5, disassembly: "AND A, L"    , byte_size: 1, clock_tick: 4 , function: and_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA6, disassembly: "AND A, (HL)" , byte_size: 1, clock_tick: 8 , function: and_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA7, disassembly: "AND A, A"    , byte_size: 1, clock_tick: 4 , function: and_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA8, disassembly: "XOR A, B"    , byte_size: 1, clock_tick: 4 , function: xor_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0xA9, disassembly: "XOR A, C"    , byte_size: 1, clock_tick: 4 , function: xor_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAA, disassembly: "XOR A, D"    , byte_size: 1, clock_tick: 4 , function: xor_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAB, disassembly: "XOR A, E"    , byte_size: 1, clock_tick: 4 , function: xor_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAC, disassembly: "XOR A, H"    , byte_size: 1, clock_tick: 4 , function: xor_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAD, disassembly: "XOR A, L"    , byte_size: 1, clock_tick: 4 , function: xor_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAE, disassembly: "XOR A, (HL)" , byte_size: 1, clock_tick: 8 , function: xor_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0xAF, disassembly: "XOR A, A"    , byte_size: 1, clock_tick: 4 , function: xor_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB0, disassembly: "OR A, B"     , byte_size: 1, clock_tick: 4 , function: or_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB1, disassembly: "OR A, C"     , byte_size: 1, clock_tick: 4 , function: or_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB2, disassembly: "OR A, D"     , byte_size: 1, clock_tick: 4 , function: or_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB3, disassembly: "OR A, E"     , byte_size: 1, clock_tick: 4 , function: or_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB4, disassembly: "OR A, H"     , byte_size: 1, clock_tick: 4 , function: or_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB5, disassembly: "OR A, L"     , byte_size: 1, clock_tick: 4 , function: or_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB6, disassembly: "OR A, (HL)"  , byte_size: 1, clock_tick: 8 , function: or_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB7, disassembly: "OR A, A"     , byte_size: 1, clock_tick: 4 , function: or_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB8, disassembly: "CP A, B"     , byte_size: 1, clock_tick: 4 , function: cp_a_b }),
    GenericInstruction::VOID(  Instruction { opcode: 0xB9, disassembly: "CP A, C"     , byte_size: 1, clock_tick: 4 , function: cp_a_c }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBA, disassembly: "CP A, D"     , byte_size: 1, clock_tick: 4 , function: cp_a_d }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBB, disassembly: "CP A, E"     , byte_size: 1, clock_tick: 4 , function: cp_a_e }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBC, disassembly: "CP A, H"     , byte_size: 1, clock_tick: 4 , function: cp_a_h }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBD, disassembly: "CP A, L"     , byte_size: 1, clock_tick: 4 , function: cp_a_l }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBE, disassembly: "CP A, (HL)"  , byte_size: 1, clock_tick: 8 , function: cp_a_hl_addr }),
    GenericInstruction::VOID(  Instruction { opcode: 0xBF, disassembly: "CP A, A"     , byte_size: 1, clock_tick: 4 , function: cp_a_a }),
    GenericInstruction::VOID(  Instruction { opcode: 0xC0, disassembly: "RET NZ"      , byte_size: 1, clock_tick: 8 , function: ret_nz }),
];

// TODO: add tests