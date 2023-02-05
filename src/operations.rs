use crate::instruction::{GenericInstruction, Instruction};
use crate::memory::Memory;

pub fn unimplemented<T>(instr: &Instruction<T>, _memory: &mut Memory, _value: T) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction<()>, _memory: &mut Memory, _value: ()) {}

pub fn ld_bc_d16(_instr: &Instruction<u16>, memory: &mut Memory, value: u16) {
    memory.registers.set_bc(value);
}

pub fn ld_bc_a(_instr: &Instruction<()>, memory: &mut Memory, _value: ()) {
    memory.set_memory_simple(memory.registers.get_bc() as usize, memory.registers.get_a());
}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [GenericInstruction; 4] = [
    GenericInstruction::VOID(Instruction { opcode: 0x00, disassembly: "NOP", operands_count: 0, clock_tick: 4, function: noop }),
    GenericInstruction::D16(Instruction { opcode: 0x01, disassembly: "LD BC, d16", operands_count: 1, clock_tick: 12, function: ld_bc_d16 }),
    GenericInstruction::VOID(Instruction { opcode: 0x02, disassembly: "LD (BC), A", operands_count: 0, clock_tick: 8, function: ld_bc_a }),
    GenericInstruction::VOID(Instruction { opcode: 0x02, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented }),
];