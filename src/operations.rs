use crate::instruction::{GenericInstruction, Instruction};
use crate::memory::Memory;
use crate::utils::{assign_bit, check_half_carry_simple_add, check_half_carry_simple_sub, check_half_carry_wide_add, get_bit};

pub fn unimplemented<T>(instr: &Instruction<T>, _memory: &mut Memory, _value: T) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction<()>, _memory: &mut Memory, _value: ()) {}

pub fn ld_bc_d16(_instr: &Instruction<u16>, memory: &mut Memory, value: u16) {
    memory.registers.set_bc(value);
}

pub fn ld_bc_addr_a(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    memory.write_simple_far_addr(memory.registers.get_bc(), memory.registers.get_a());
}

pub fn inc_bc(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    memory.registers.set_bc(memory.registers.get_bc() + 1);
}

pub fn inc_b(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    let old_value = memory.registers.get_b();
    let new_value = old_value + 1;
    memory.registers.set_b(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 3.
    memory.registers.set_half_carry_flag(check_half_carry_simple_add(old_value, 1));
}

pub fn dec_b(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    let old_value = memory.registers.get_b();
    let new_value = old_value - 1;
    memory.registers.set_b(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    // H => Set if borrow from bit 4.
    memory.registers.set_half_carry_flag(check_half_carry_simple_sub(old_value, 1));
}

pub fn ld_b_d8(_instr: &Instruction<u8>, memory: &mut Memory, value: u8) {
    memory.registers.set_b(value);
}

// TODO: Check
pub fn rlca(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    /*
        RLCA is "Rotate left circular register A"

        It shifts left the register A by one
        Popped value is place in carry flag & return to the lowest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, 7);


    // Left shift and put back top bit in the lowest bit
    memory.registers.set_a(assign_bit(old_value << 1, 0, popped_value));

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn ld_a16_addr_sp(_instr: &Instruction<u16>, memory: &mut Memory, value: u16) {
    memory.write_wide_far_addr(value, memory.registers.SP);
}

pub fn add_hl_bc(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    let old_value = memory.registers.get_hl();
    let bc_value = memory.registers.get_bc();

    let result = old_value.overflowing_add(bc_value);

    memory.registers.set_hl(result.0);

    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 11.
    memory.registers.set_half_carry_flag(check_half_carry_wide_add(old_value, bc_value));
    // C => Set if overflow from bit 15.
    memory.registers.set_carry_flag(result.1);
}

pub fn ld_a_bc_addr(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    memory.registers.set_a(memory.read_simple_far_addr(memory.registers.get_bc()));
}

pub fn dec_bc(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    memory.registers.set_bc(memory.registers.get_bc() - 1);
}

pub fn inc_c(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    let old_value = memory.registers.get_c();
    let new_value = old_value + 1;
    memory.registers.set_c(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(false);
    // H => Set if overflow from bit 3.
    memory.registers.set_half_carry_flag(check_half_carry_simple_add(old_value, 1));
}

pub fn dec_c(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    let old_value = memory.registers.get_c();
    let new_value = old_value - 1;
    memory.registers.set_c(new_value);

    // Operation flags
    memory.registers.set_zero_flag(new_value == 0);
    memory.registers.set_subtraction_flag(true);
    // H => Set if borrow from bit 4.
    memory.registers.set_half_carry_flag(check_half_carry_simple_sub(old_value, 1));
}

pub fn ld_c_d8(_instr: &Instruction<u8>, memory: &mut Memory, value: u8) {
    memory.registers.set_c(value);
}

// TODO: Check
pub fn rrca(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    /*
        RLCA is "Rotate right circular register A"

        It shifts right the register A by one
        Popped value is place in carry flag & return to the highest bit
    */

    let old_value = memory.registers.get_a();
    let popped_value = get_bit(old_value, 0);


    // Left shift and put back top bit in the lowest bit
    memory.registers.set_a(assign_bit(old_value >> 1, 7, popped_value));

    memory.registers.set_zero_flag(false);
    memory.registers.set_subtraction_flag(false);
    memory.registers.set_half_carry_flag(false);
    memory.registers.set_carry_flag(popped_value);
}

pub fn stop(_instr: &Instruction<u8>, _memory: &mut Memory, _value: u8) {
    // TODO: Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.
    todo!("Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.")
}

pub fn ld_de_d16(_instr: &Instruction<u16>, memory: &mut Memory, value: u16) {
    memory.registers.set_de(value);
}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [GenericInstruction; 18] = [
    GenericInstruction::VOID(Instruction { opcode: 0x00, disassembly: "NOP", byte_size: 1, operands_count: 0, clock_tick: 4, function: noop }),
    GenericInstruction::D16(Instruction { opcode: 0x01, disassembly: "LD BC, d16", byte_size: 3, operands_count: 1, clock_tick: 12, function: ld_bc_d16 }),
    GenericInstruction::VOID(Instruction { opcode: 0x02, disassembly: "LD (BC), A", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_bc_addr_a }),
    GenericInstruction::VOID(Instruction { opcode: 0x03, disassembly: "INC BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: inc_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x04, disassembly: "INC B", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_b }),
    GenericInstruction::VOID(Instruction { opcode: 0x05, disassembly: "DEC B", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_b }),
    GenericInstruction::D8(Instruction { opcode: 0x06, disassembly: "LD B, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_b_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x07, disassembly: "RLCA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rlca }),
    GenericInstruction::A16(Instruction { opcode: 0x08, disassembly: "LD (a16), SP", byte_size: 3, operands_count: 1, clock_tick: 20, function: ld_a16_addr_sp }),
    GenericInstruction::VOID(Instruction { opcode: 0x09, disassembly: "AD HL, BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: add_hl_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x0A, disassembly: "LD A, (BC)", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_a_bc_addr }),
    GenericInstruction::VOID(Instruction { opcode: 0x0B, disassembly: "DEC BC", byte_size: 1, operands_count: 0, clock_tick: 8, function: dec_bc }),
    GenericInstruction::VOID(Instruction { opcode: 0x0C, disassembly: "INC C", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_c }),
    GenericInstruction::VOID(Instruction { opcode: 0x0D, disassembly: "DEC C", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_c }),
    GenericInstruction::D8(Instruction { opcode: 0x0E, disassembly: "LD C, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_c_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x0F, disassembly: "RRCA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rrca }),
    GenericInstruction::D8(Instruction { opcode: 0x10, disassembly: "STOP d8", byte_size: 2, operands_count: 1, clock_tick: 4, function: unimplemented }),
    GenericInstruction::D16(Instruction { opcode: 0x11, disassembly: "LD DE, d16", byte_size: 3, operands_count: 1, clock_tick: 12, function: ld_de_d16 }),
];