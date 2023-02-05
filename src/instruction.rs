use std::fmt::{Debug, Formatter, Result};

use crate::operations::INSTRUCTIONS;
use crate::log;
use crate::memory::Memory;

pub type OpCode = u8;
pub type InstructionFn = fn(&Instruction, &mut Memory, values: Values) -> ();

#[derive(Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub disassembly: &'static str,
    pub operands_count: usize,
    pub clock_tick: u8,
    pub function: InstructionFn,
}

#[derive(Clone, Copy)]
pub union Values {
    pub u3: u8,
    pub d8: u8,
    pub d16: u16,
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
        debug_assert!(opcode < INSTRUCTIONS.len());
        return INSTRUCTIONS[opcode].clone();
    }

    pub fn execute(&self, memory: &mut Memory, values: Values) {
        self.log();
        (self.function)(self, memory, values);
    }
}
