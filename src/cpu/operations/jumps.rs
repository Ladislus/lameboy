use crate::cpu::memory::Memory;
use crate::utils::types::{AddressOffset, FarAddress, Void};

//  #############################
//  #         Template          #
//  #############################

// TODO: Check
fn template_rst(memory: &mut Memory, value: FarAddress) {
    memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
    memory.registers.PC = value;
}

//  #############################
//  #           Call            #
//  #############################

pub fn call_a16(memory: &mut Memory, value: FarAddress) {
    memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
    memory.registers.PC = value;
}

pub fn call_z_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_zero_flag() { call_a16(memory, value); }
}

pub fn call_nz_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_zero_flag() { call_a16(memory, value); }
}

pub fn call_c_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_carry_flag() { call_a16(memory, value); }
}

pub fn call_nc_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_carry_flag() { call_a16(memory, value); }
}

//  #############################
//  #          Returns          #
//  #############################

// TODO: Check
pub fn ret(memory: &mut Memory, _value: Void) {
    memory.registers.PC = memory.stack.pop_wide(&mut memory.registers.SP);
}

pub fn reti(memory: &mut Memory, _value: Void) {
    // https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/#RETI
    // Return from subroutine and enable interrupts. This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
    // TODO Enable interrupt
    ret(memory, _value);
}

pub fn ret_z(memory: &mut Memory, _value: Void) {
    if memory.registers.get_zero_flag() { ret(memory, _value) }
}

pub fn ret_nz(memory: &mut Memory, _value: Void) {
    if !memory.registers.get_zero_flag() { ret(memory, _value); }
}

pub fn ret_c(memory: &mut Memory, _value: Void) {
    if memory.registers.get_carry_flag() { ret(memory, _value); }
}

pub fn ret_nc(memory: &mut Memory, _value: Void) {
    if !memory.registers.get_carry_flag() { ret(memory, _value); }
}

//  #############################
//  #       Absolute Jump       #
//  #############################

pub fn jp_a16(memory: &mut Memory, value: FarAddress) {
    memory.registers.PC = value;
}

pub fn jp_z_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_zero_flag() { jp_a16(memory, value); }
}

pub fn jp_nz_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_zero_flag() { jp_a16(memory, value); }
}

pub fn jp_c_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_carry_flag() { jp_a16(memory, value); }
}

pub fn jp_nc_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_carry_flag() { jp_a16(memory, value); }
}

//  #############################
//  #       Relative Jump       #
//  #############################

pub fn jr_r8(memory: &mut Memory, value: AddressOffset) {
    // Relative Jump to address n16. The address is encoded as a signed 8-bit offset from the address immediately following the JR instruction, so the target address n16 must be between -128 and 127 bytes away.
    // For example:
    //     JR Label  ; no-op; encoded offset of 0
    // Label:
    //     JR Label  ; infinite loop; encoded offset of -2
    // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
    memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
}

pub fn jr_z_r8(memory: &mut Memory, value: AddressOffset) {
    if memory.registers.get_zero_flag() { jr_r8(memory, value); }
}

pub fn jr_nz_r8(memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_zero_flag() { jr_r8(memory, value); }
}

pub fn jr_c_r8(memory: &mut Memory, value: AddressOffset) {
    if memory.registers.get_carry_flag() { jr_r8(memory, value); }
}

pub fn jr_nc_r8(memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_carry_flag() { jr_r8(memory, value); }
}

//  #############################
//  #           Reset           #
//  #############################

pub fn rst_00h(memory: &mut Memory, _value: Void) {
    template_rst(memory, 0x00);
}

pub fn rst_08h(memory: &mut Memory, _value: Void) {
    template_rst(memory, 0x08);
}

pub fn rst_10h(memory: &mut Memory, _value: Void) {
    template_rst(memory, 0x10);
}

pub fn rst_18h(memory: &mut Memory, _value: Void) {
    template_rst(memory, 0x18);
}