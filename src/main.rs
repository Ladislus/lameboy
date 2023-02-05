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
use crate::instruction::{Instruction, Values};

fn main() {
    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);
    let instr = Instruction::fetch(1);

    instr.execute(&mut memory, Values { d16: 1 });

    launch_gui();
}
