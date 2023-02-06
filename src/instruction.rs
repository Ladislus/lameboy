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
    pub byte_size: usize,
    pub operands_count: usize,
    pub clock_tick: u8,
    pub function: InstructionFn<T>,
}

#[derive(Clone, Copy, Debug)]
pub enum GenericInstruction {
    /// Doesn't take any operand
    VOID(Instruction<()>),
    /// Takes 8-bit data operand
    D8(Instruction<u8>),
    /// Takes 16-bit little-endian data
    D16(Instruction<u16>),
    /// Takes 8-bit data (offset for $FF00)
    A8(Instruction<u8>),
    /// Takes 16-bit little endian address
    A16(Instruction<u16>),
    /// Takes 8-bit signed data
    R8(Instruction<i8>)
}

impl<T> Debug for Instruction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Instruction")
            .field("OP Code", &format_args!("{:#04X}", self.opcode))
            .field("ASM Name", &self.disassembly)
            .field("Byte size", &self.byte_size)
            .field("Number of operands", &self.operands_count)
            .field("CPU Clock ticks", &self.clock_tick)
            .finish_non_exhaustive()
    }
}

impl<T> Instruction<T> {
    pub fn execute(&self, memory: &mut Memory, value: T) {
        log!("INSTRUCTION", format!("Executing {:?}", self));
        (self.function)(self, memory, value);
    }
}

pub fn instruction_from_opcode(opcode: OpCode) -> GenericInstruction {
    let opcode = opcode as usize;
    debug_assert!(opcode < INSTRUCTIONS.len());
    return INSTRUCTIONS[opcode];
}
