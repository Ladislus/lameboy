use crate::cpu::instruction::{GenericInstruction, instruction_from_opcode, OpCode};
use crate::cpu::memory::Memory;
use crate::cpu::operations::INSTRUCTIONS;
use crate::gui::gui::launch_gui;

mod cpu;
mod gui;
mod utils;

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
