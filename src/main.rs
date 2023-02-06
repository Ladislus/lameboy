extern crate core;

mod memory;
mod gui;
mod sound;
mod utils;
mod register;
mod instruction;
mod operations;

use crate::gui::launch_gui;
use crate::memory::Memory;
use crate::instruction::{instruction_from_opcode, GenericInstruction, OpCode};
use crate::operations::INSTRUCTIONS;

fn main() {
    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);

    // TODO: Remove after test
    for idx in 0..INSTRUCTIONS.len() {
        match instruction_from_opcode(idx as OpCode) {
            GenericInstruction::VOID(instr) => instr.execute(&mut memory, ()),
            GenericInstruction::D16(instr) => instr.execute(&mut memory, 1),
            GenericInstruction::D8(instr) => instr.execute(&mut memory, 1),
            GenericInstruction::A16(instr) => instr.execute(&mut memory, 100),
            instr @ _ => unimplemented!("GenericInstruction {:?} not implemented", instr)
        }
    }

    launch_gui();
}
