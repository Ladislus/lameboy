use crate::cpu::memory::Memory;
use crate::utils::types::{AddressOffset, FarAddress, Void};

//  #############################
//  #           Call            #
//  #############################

pub fn call_a16(memory: &mut Memory, value: FarAddress) {
    memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
    memory.registers.PC = value;
}

pub fn call_z_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_zero_flag() {
        memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
        memory.registers.PC = value;
    }
}

pub fn call_nz_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_zero_flag() {
        memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
        memory.registers.PC = value;
    }
}

//  #############################
//  #          Returns          #
//  #############################

// TODO: Check
pub fn ret(memory: &mut Memory, _value: Void) {
    memory.registers.PC = memory.stack.pop_wide(&mut memory.registers.SP);
}

// TODO: Check
pub fn ret_z(memory: &mut Memory, _value: Void) {
    if memory.registers.get_zero_flag() {
        memory.registers.PC = memory.stack.pop_wide(&mut memory.registers.SP);
    }
}

// TODO: Check
pub fn ret_nz(memory: &mut Memory, _value: Void) {
    if !memory.registers.get_zero_flag() {
        memory.registers.PC = memory.stack.pop_wide(&mut memory.registers.SP);
    }
}

//  #############################
//  #       Absolute Jump       #
//  #############################

pub fn jp_a16(memory: &mut Memory, value: FarAddress) {
    memory.registers.PC = value;
}

pub fn jp_z_a16(memory: &mut Memory, value: FarAddress) {
    if memory.registers.get_zero_flag() {
        memory.registers.PC = value;
    }
}

pub fn jp_nz_a16(memory: &mut Memory, value: FarAddress) {
    if !memory.registers.get_zero_flag() {
        memory.registers.PC = value;
    }
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
    if memory.registers.get_zero_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn jr_nz_r8(memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_zero_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn jr_c_r8(memory: &mut Memory, value: AddressOffset) {
    if memory.registers.get_carry_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

pub fn jr_nc_r8(memory: &mut Memory, value: AddressOffset) {
    if !memory.registers.get_carry_flag() {
        // To do "safe" signed + unsigned operation, do a wrapping add with both operands interpreted as unsigned
        memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
    }
}

//  #############################
//  #           Reset           #
//  #############################

// TODO: Check
pub fn rst_00h(memory: &mut Memory, _value: Void) {
    memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
    memory.registers.PC = 0;
}

// TODO: Check
pub fn rst_08h(memory: &mut Memory, _value: Void) {
    memory.stack.push_wide(&mut memory.registers.SP, memory.registers.PC);
    memory.registers.PC = 0x08;
}