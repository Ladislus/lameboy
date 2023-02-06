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
use crate::instruction::{GenericInstruction, OpCode, instruction_from_opcode};
use crate::operations::INSTRUCTIONS;

const PROGRAM_NAME: &'static str = "LameBoy";
const PROGRAM_VERSION: &'static str = "0.0.1";

fn main() {

    log!("PROGRAM", format!("{} v{}", PROGRAM_NAME, PROGRAM_VERSION));

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
