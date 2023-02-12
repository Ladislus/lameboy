use crate::cpu::instruction::{GenericInstruction, instruction_from_opcode};
use crate::cpu::memory::Memory;
use crate::gui::gui::launch_gui;
use crate::utils::log::log;

mod cpu;
mod gui;
mod utils;

const PROGRAM_NAME: &'static str = "LameBoy";
const PROGRAM_VERSION: &'static str = "0.0.1";

fn main() {

    log!("PROGRAM", format!("{} v{}", PROGRAM_NAME, PROGRAM_VERSION));

    // Allocate 1024 bytes of memory
    let mut memory = Memory::new(1024);
    // memory.registers.set_a(0b1100_0000);
    // memory.registers.set_carry_flag(false);

    match instruction_from_opcode(0x27) {
        GenericInstruction::VOID(instr) => instr.execute(&mut memory, ()),
        GenericInstruction::DATA16(instr) => instr.execute(&mut memory, 1),
        GenericInstruction::DATA8(instr) => instr.execute(&mut memory, 1),
        GenericInstruction::ADDR16(instr) => instr.execute(&mut memory, 100),
        instr @ _ => unimplemented!("GenericInstruction {:?} not implemented", instr)
    }

    launch_gui();
}