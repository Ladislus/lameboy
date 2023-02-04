use std::fmt::{Debug, Formatter, Result};

use crate::operations::INSTRUCTION_INFORMATION;
use crate::log;
use crate::memory::Memory;

pub type OpCode = u8;
pub type InstructionFn = fn(&Instruction, &mut Memory);

#[derive(Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub disassembly: &'static str,
    pub operands_count: usize,
    pub clock_tick: u8,
    pub function: InstructionFn,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Instruction")
            .field("OP Code", &format_args!("{:#04X}", self.opcode))
            .field("ASM Name", &self.disassembly)
            .field("Number of operands", &self.operands_count)
            .field("Clock ticks", &self.clock_tick)
            .finish()
    }
}

impl Instruction {
    #[cfg(debug_assertions)]
    fn log(&self) {
        log!("INSTRUCTION", format!("Running {:?}", self));
    }

    #[cfg(not(debug_assertions))]
    fn log(&self) {}

    pub fn fetch(opcode: OpCode) -> Instruction {
        let opcode = opcode as usize;
        debug_assert!(opcode < INSTRUCTION_INFORMATION.len());
        return INSTRUCTION_INFORMATION[opcode].clone();
    }

    pub fn execute(&self, memory: &mut Memory) {
        self.log();
        (self.function)(self, memory);
    }
}
