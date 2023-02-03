use crate::instruction::Instruction;
use crate::memory::Memory;

pub fn unimplemented(instr: &Instruction, _memory: &mut Memory) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction, mut _memory: &mut Memory) {}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTION_INFORMATION: [Instruction; 17] = [
    Instruction { opcode: 0x00, disassembly: "NOP", operands_count: 0, clock_tick: 4, function: noop },
    Instruction { opcode: 0x01, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x02, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x03, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x04, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x05, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x06, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x07, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x08, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x09, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0A, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0B, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0C, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0D, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0E, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x0F, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
    Instruction { opcode: 0x10, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
];