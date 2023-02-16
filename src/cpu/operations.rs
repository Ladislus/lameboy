use crate::cpu::instruction::{GenericInstruction, Instruction, WideValueInstruction, ValueInstruction, FarAddressInstruction, OffsetInstruction, VoidInstruction};
use crate::cpu::memory::Memory;
use crate::cpu::template::{template_add_hl, template_dec_value, template_dec_wide, template_inc_value, template_inc_wide, template_ld};
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

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [GenericInstruction; 48] = [
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
    GenericInstruction::OFFSET(Instruction { opcode: 0x18, disassembly: "JR r8", byte_size: 2, operands_count: 1, clock_tick: 12, function: jr_r8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x19, disassembly: "ADD HL, DE", byte_size: 1, operands_count: 0, clock_tick: 8, function: add_hl_de }),
    GenericInstruction::VOID(Instruction { opcode: 0x1A, disassembly: "LD A, (DE)", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_a_de_addr }),
    GenericInstruction::VOID(Instruction { opcode: 0x1B, disassembly: "DEC DE", byte_size: 1, operands_count: 0, clock_tick: 8, function: dec_de }),
    GenericInstruction::VOID(Instruction { opcode: 0x1C, disassembly: "INC E", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_e }),
    GenericInstruction::VOID(Instruction { opcode: 0x1D, disassembly: "DEC E", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_e }),
    GenericInstruction::DATA8(Instruction { opcode: 0x1E, disassembly: "LD E, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_e_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x1F, disassembly: "RRA", byte_size: 1, operands_count: 0, clock_tick: 4, function: rra }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x20, disassembly: "JR NZ, r8", byte_size: 1, operands_count: 1, clock_tick: 8, function: jr_nz_r8 }),
    GenericInstruction::DATA16(Instruction { opcode: 0x21, disassembly: "LD HL, d16", byte_size: 3, operands_count: 1, clock_tick: 12, function: ld_hl_d16 }),
    GenericInstruction::VOID(Instruction { opcode: 0x22, disassembly: "LD (HL+), A", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_hli_addr_a }),
    GenericInstruction::VOID(Instruction { opcode: 0x23, disassembly: "INC HL", byte_size: 1, operands_count: 0, clock_tick: 8, function: inc_hl }),
    GenericInstruction::VOID(Instruction { opcode: 0x24, disassembly: "INC H", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_h }),
    GenericInstruction::VOID(Instruction { opcode: 0x25, disassembly: "DEC H", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_h }),
    GenericInstruction::DATA8(Instruction { opcode: 0x26, disassembly: "LD H, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_h_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x27, disassembly: "DAA", byte_size: 1, operands_count: 0, clock_tick: 4, function: daa }),
    GenericInstruction::OFFSET(Instruction { opcode: 0x28, disassembly: "JR Z, r8", byte_size: 2, operands_count: 1, clock_tick: 8, function: jr_z_r8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x29, disassembly: "ADD HL, HL", byte_size: 1, operands_count: 0, clock_tick: 8, function: add_hl_hl }),
    GenericInstruction::VOID(Instruction { opcode: 0x2A, disassembly: "LD A, (HL+)", byte_size: 1, operands_count: 0, clock_tick: 8, function: ld_a_hli_addr }),
    GenericInstruction::VOID(Instruction { opcode: 0x2B, disassembly: "DEC HL", byte_size: 1, operands_count: 0, clock_tick: 8, function: dec_hl }),
    GenericInstruction::VOID(Instruction { opcode: 0x2C, disassembly: "INC L", byte_size: 1, operands_count: 0, clock_tick: 4, function: inc_l }),
    GenericInstruction::VOID(Instruction { opcode: 0x2D, disassembly: "DEC L", byte_size: 1, operands_count: 0, clock_tick: 4, function: dec_l }),
    GenericInstruction::DATA8(Instruction { opcode: 0x2E, disassembly: "LD L, d8", byte_size: 2, operands_count: 1, clock_tick: 8, function: ld_l_d8 }),
    GenericInstruction::VOID(Instruction { opcode: 0x2F, disassembly: "CPL", byte_size: 1, operands_count: 0, clock_tick: 4, function: cpl }),
];

// TODO: add tests