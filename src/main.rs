mod memory;
mod gui;
mod sound;
mod utils;
mod register;
mod instruction;
mod operations;

use crate::gui::launch_gui;
use crate::memory::Memory;
use crate::instruction::Instruction;

fn main() {
    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);
    let instr = Instruction::fetch(0);

    instr.execute(&mut memory);

    launch_gui();
}
