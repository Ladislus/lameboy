use std::fmt::{Debug, Formatter, Result};

use crate::operations::INSTRUCTIONS;
use crate::log;
use crate::memory::Memory;

pub type OpCode = u8;
pub type InstructionFn<T> = fn(&Instruction<T>, &mut Memory, values: T);

#[derive(Clone, Copy)]
pub struct Instruction<T> {
    pub opcode: OpCode,
    pub disassembly: &'static str,
    pub operands_count: usize,
    pub clock_tick: u8,
    pub function: InstructionFn<T>,
}

#[derive(Clone, Copy)]
pub enum GenericInstruction {
    VOID(Instruction<()>),
    D8(Instruction<u8>),
    D16(Instruction<u16>),
    A8(Instruction<u8>),
    A16(Instruction<u16>),
    R8(Instruction<i8>)
}

impl<T> Debug for Instruction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Instruction")
            .field("OP Code", &format_args!("{:#04X}", self.opcode))
            .field("ASM Name", &self.disassembly)
            .field("Number of operands", &self.operands_count)
            .field("Clock ticks", &self.clock_tick)
            .finish_non_exhaustive()
    }
}

impl<T> Instruction<T> {
    #[cfg(debug_assertions)]
    fn log(&self) {
        log!("INSTRUCTION", format!("Running {:?}", self));
    }

    #[cfg(not(debug_assertions))]
    fn log(&self) {}

    pub fn execute(&self, memory: &mut Memory, value: T) {
        self.log();
        (self.function)(self, memory, value);
    }
}

pub fn instruction_from_opcode(opcode: OpCode) -> GenericInstruction {
    let opcode = opcode as usize;
    debug_assert!(opcode < INSTRUCTIONS.len());
    return INSTRUCTIONS[opcode];
}
