use crate::cpu::instruction::{GenericInstruction, instruction_from_opcode, PREFIXED_OPCODE, OpCode};
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
    let mut prefixed_encountered = false;

    // TODO: Remove
    // LD A => PREFIX => SWAP A
    let temp_opcodes: Vec<OpCode> = vec![0x3E, 0xCB, 0x37];


    // TODO fetch opcode from GB game
    for opcode in temp_opcodes {

        if opcode == PREFIXED_OPCODE {
            log!("INSTRUCTION", "Encountered prefix instruction");
            prefixed_encountered = true;
        } else {
            match instruction_from_opcode(opcode, prefixed_encountered) {
                GenericInstruction::VOID(instr) => instr.execute(&mut memory, ()),
                GenericInstruction::WIDE(instr) => {
                    // TODO: Parse next two bytes as value
                    instr.execute(&mut memory, 1);
                },
                GenericInstruction::VALUE(instr) => {
                    // TODO: Parse next byte as value
                    instr.execute(&mut memory, 0b1111_0000);
                },
                GenericInstruction::FAR(instr) => {
                    // TODO: Parse next two bytes as value
                    instr.execute(&mut memory, 0xFF00);
                },
                GenericInstruction::NEAR(instr) => {
                    // TODO: Parse next byte as value
                    instr.execute(&mut memory, 0x0F);
                },
                GenericInstruction::OFFSET(instr) => {
                    // TODO: Parse next byte as value
                    instr.execute(&mut memory, -2);
                },
            }

            prefixed_encountered = false;
        }
    }

    launch_gui();
}