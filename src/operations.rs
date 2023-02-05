use sdl2::libc::exit;
use sdl2::log::log;
use crate::instruction::{Instruction, Values};
use crate::log;
use crate::memory::Memory;

pub fn unimplemented(instr: &Instruction, _memory: &mut Memory, _values: Values) {
    unimplemented!("Function for {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction, mut _memory: &mut Memory, _values: Values) {}

pub fn ld_bc_d16(instr: &Instruction, mut memory: &mut Memory, values: Values) {
    unsafe { memory.registers.BC.wide = values.d16; }
}

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTIONS: [Instruction; 3] = [
    Instruction { opcode: 0x00, disassembly: "NOP", operands_count: 0, clock_tick: 4, function: noop },
    Instruction { opcode: 0x01, disassembly: "LD BC, d16", operands_count: 1, clock_tick: 0, function: ld_bc_d16 },
    Instruction { opcode: 0x02, disassembly: "", operands_count: 0, clock_tick: 0, function: unimplemented },
];