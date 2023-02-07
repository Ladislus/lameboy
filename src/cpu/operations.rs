use crate::cpu::instruction::{GenericInstruction, Instruction};
use crate::cpu::memory::Memory;
use crate::log;
use crate::utils::bits::{assign_bit, bit_size, check_half_carry_add, check_half_carry_sub, check_half_carry_wide_add, get_bit};
use crate::utils::types::{FarAddress, OffsetAddress, Value, Void, WideValue};

pub fn unimplemented<T>(instr: &Instruction<T>, _memory: &mut Memory, _value: T) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction<Void>, _memory: &mut Memory, _value: Void) {}

pub fn ld_bc_d16(_instr: &Instruction<WideValue>, memory: &mut Memory, value: WideValue) {
    memory.registers.set_bc(value);
}

pub fn ld_bc_addr_a(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_bc(), memory.registers.get_a());
}

pub fn inc_bc(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.registers.set_bc(memory.registers.get_bc() + 1);
}

pub fn inc_b(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_b();
    let new_value = old_value + 1;
    memory.registers.set_b(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 3.
    memory.registers.set_half_carry_flag(check_half_carry_add(old_value, 1));
}

pub fn dec_b(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_b();
    let new_value = old_value - 1;
    memory.registers.set_b(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    // H => Set if borrow from bit 4.
    memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, 1));
}

pub fn ld_b_d8(_instr: &Instruction<Value>, memory: &mut Memory, value: Value) {
    memory.registers.set_b(value);
}

// TODO: Check
pub fn rlca(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    /*
        RLCA is "Rotate left circular register A"

        It shifts left the register A by one
        Popped value is place in carry flag & return to the lowest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, bit_size(old_value) - 1);
    // Left shift and put back top bit in the lowest bit
    let new_value = assign_bit(old_value << 1, 0, popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn ld_a16_addr_sp(_instr: &Instruction<FarAddress>, memory: &mut Memory, value: FarAddress) {
    memory.write_wide_far_addr(value, memory.registers.SP);
}

pub fn add_hl_bc(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_hl();
    let bc_value = memory.registers.get_bc();

    let (result, has_overflown) = old_value.overflowing_add(bc_value);

    memory.registers.set_hl(result);

    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 11.
    memory.registers.set_half_carry_flag(check_half_carry_wide_add(old_value, bc_value));
    // C => Set if overflow from bit 15.
    memory.registers.set_carry_flag(has_overflown);
}

pub fn ld_a_bc_addr(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_bc()));
}

pub fn dec_bc(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.registers.set_bc(memory.registers.get_bc() - 1);
}

pub fn inc_c(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_c();
    let new_value = old_value + 1;
    memory.registers.set_c(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 3.
    memory.registers.set_half_carry_flag(check_half_carry_add(old_value, 1));
}

pub fn dec_c(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_c();
    let new_value = old_value - 1;
    memory.registers.set_c(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    // H => Set if borrow from bit 4.
    memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, 1));
}

pub fn ld_c_d8(_instr: &Instruction<Value>, memory: &mut Memory, value: Value) {
    memory.registers.set_c(value);
}

// TODO: Check
pub fn rrca(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    /*
        RLCA is "Rotate right circular register A"

        It shifts right the register A by one
        Popped value is place in carry flag & return to the highest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, 0);
    let new_value = assign_bit(old_value >> 1, bit_size(old_value) - 1, popped_value);

    log!("OPERATION", format!("{:#0width$b} => {:#0width$b} + carry: {}", old_value, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    // Left shift and put back top bit in the lowest bit
    memory.registers.set_a(new_value);

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn stop(_instr: &Instruction<u8>, _memory: &mut Memory, _value: u8) {
    todo!("Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.")
}

pub fn ld_de_d16(_instr: &Instruction<WideValue>, memory: &mut Memory, value: WideValue) {
    memory.registers.set_de(value);
}

pub fn ld_de_addr_a(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.write_far_addr(memory.registers.get_de(), memory.registers.get_a());
}

pub fn inc_de(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.registers.set_de(memory.registers.get_de() + 1);
}

pub fn inc_d(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_d();
    let new_value = old_value + 1;
    memory.registers.set_d(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 3.
    memory.registers.set_half_carry_flag(check_half_carry_add(old_value, 1));
}

pub fn dec_d(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_d();
    let new_value = old_value - 1;
    memory.registers.set_d(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    // H => Set if borrow from bit 4.
    memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, 1));
}

pub fn ld_d_d8(_instr: &Instruction<Value>, memory: &mut Memory, value: Value) {
    memory.registers.set_d(value);
}

pub fn rla(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_a();
    let old_carry = memory.registers.get_carry_flag();
    let popped_value = get_bit(old_value, bit_size(old_value) - 1);
    let new_value = assign_bit(old_value << 1, 0, old_carry);

    log!("OPERATION", format!("{:#0width$b} + carry: {} => {:#0width$b} + carry: {}", old_value, popped_value as u8, new_value, popped_value as u8, width = bit_size(old_value) + 2));

    memory.registers.set_a(new_value);
    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn jp_r8(_instr: &Instruction<OffsetAddress>, memory: &mut Memory, value: OffsetAddress) {
    // Relative Jump to address n16. The address is encoded as a signed 8-bit offset from the address immediately following the JR instruction, so the target address n16 must be between -128 and 127 bytes away.
    // For example:
    //     JR Label  ; no-op; encoded offset of 0
    // Label:
    //     JR Label  ; infinite loop; encoded offset of -2
    // TODO: Check
    memory.registers.PC = memory.registers.PC.wrapping_add(value as FarAddress);
}

pub fn add_hl_de(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    let old_value = memory.registers.get_hl();
    let de_value = memory.registers.get_de();

    let (result, has_overflown) = old_value.overflowing_add(de_value);

    memory.registers.set_hl(result);

    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 11.
    memory.registers.set_half_carry_flag(check_half_carry_wide_add(old_value, de_value));
    // C => Set if overflow from bit 15.
    memory.registers.set_carry_flag(has_overflown);
}

pub fn ld_a_de_addr(_instr: &Instruction<Void>, memory: &mut Memory, _value: Void) {
    memory.registers.set_a(memory.read_far_addr(memory.registers.get_de()));
}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [GenericInstruction; 27] = [
    GenericInstruction::VOID(Instruction { opcode: 0x00, disassembly: "NOP", byte_size: 1, operands_count: 0, clock_tick: 4, function: noop }),
    GenericInstruction::DATA16(Instruction { opcode: 0x01, disassembly: "LD BC, d16", byte_size: 3, operands_count: 1, clock_tick: 12, function: ld_bc_d16 }),
    GenericInstruction::VOID(Instruction { opcode: 0x02, disassembly: "LD (BC), A", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_bc_addr_a }),
    GenericInstruction::VOID(Instruction { opcode: 0x03, disassembly: "INC BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: inc_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x04, disassembly: "INC B", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_b }),
    GenericInstruction::VOID(Instruction { opcode: 0x05, disassembly: "DEC B", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_b }),
    GenericInstruction::DATA8(Instruction { opcode: 0x06, disassembly: "LD B, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_b_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x07, disassembly: "RLCA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rlca }),
    GenericInstruction::ADDR16(Instruction { opcode: 0x08, disassembly: "LD (a16), SP", byte_size: 3, operands_count: 1, clock_tick: 20, function: ld_a16_addr_sp }),
    GenericInstruction::VOID(Instruction { opcode: 0x09, disassembly: "AD HL, BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: add_hl_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x0A, disassembly: "LD A, (BC)", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_a_bc_addr }),
    GenericInstruction::VOID(Instruction { opcode: 0x0B, disassembly: "DEC BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: dec_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x0C, disassembly: "INC C", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_c }),
    GenericInstruction::VOID(Instruction { opcode: 0x0D, disassembly: "DEC C", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_c }),
    GenericInstruction::DATA8(Instruction { opcode: 0x0E, disassembly: "LD C, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_c_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x0F, disassembly: "RRCA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rrca }),
    GenericInstruction::DATA8(Instruction { opcode: 0x10, disassembly: "STOP d8", byte_size: 2, operands_count: 1, clock_tick: 4, function: stop }),
    GenericInstruction::DATA16(Instruction { opcode: 0x11, disassembly: "LD DE, d16", byte_size: 3, operands_count: 1, clock_tick: 12, function: ld_de_d16 }),
    GenericInstruction::VOID(Instruction { opcode: 0x12, disassembly: "LD (DE), A", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_de_addr_a }),
    GenericInstruction::VOID(Instruction { opcode: 0x13, disassembly: "INC DE", byte_size: 1, operands_count: 0, clock_tick: 8, function: inc_de }),
    GenericInstruction::VOID(Instruction { opcode: 0x14, disassembly: "INC D", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_d }),
    GenericInstruction::VOID(Instruction { opcode: 0x15, disassembly: "DEC D", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_d }),
    GenericInstruction::DATA8(Instruction { opcode: 0x16, disassembly: "LD D, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_d_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x17, disassembly: "RLA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rla }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x18, disassembly: "JR r8", byte_size: 2, operands_count: 1, clock_tick: 12, function: jp_r8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x19, disassembly: "ADD HL, DE", byte_size: 1, operands_count: 0, clock_tick: 8, function: add_hl_de }),
    GenericInstruction::VOID(Instruction { opcode: 0x1A, disassembly: "LD A, (DE)", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_a_de_addr }),
];

// TODO: add tests