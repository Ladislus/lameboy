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
use crate::instruction::{instruction_from_opcode, GenericInstruction};

fn main() {
    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);
    match instruction_from_opcode(1) {
        GenericInstruction::VOID(instr) => instr.execute(&mut memory, ()),
        GenericInstruction::D16(instr) => instr.execute(&mut memory, 1),
        _ => unimplemented!("GenericInstruction not implemented")
    }

    launch_gui();
}
