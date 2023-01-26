mod instructions;
mod memory;
mod gui;
mod sound;
mod utils;
mod register;

use crate::memory::Memory;
use instructions::instruction::Instruction;

fn main() {
    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);
    let instr = Instruction::fetch(0x00);

    instr.execute(&mut memory);
}
