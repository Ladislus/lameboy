use crate::instructions::instruction::Instruction;
use crate::memory::Memory;

pub fn unimplemented(instr: &Instruction, _memory: &mut Memory) {
    unimplemented!("Function for instruction {:?} is not implemented", instr);
}

pub fn noop(_instr: &Instruction, mut _memory: &mut Memory) {}