use crate::cpu::memory::Memory;
use crate::cpu::operations::operations::INSTRUCTIONS;
use crate::utils::log::log;
use crate::utils::types::{FarAddress, AddressOffset, Value, WideValue, NearAddress};

pub type OpCode = u8;
pub type InstructionFn<T> = fn(&mut Memory, value: T);

#[derive(Clone, Copy)]
pub struct Instruction<T> {
    pub opcode: OpCode,
    pub disassembly: &'static str,
    pub byte_size: usize,
    pub clock_tick: u8,
    pub function: InstructionFn<T>,
}

#[derive(Clone, Copy, Debug)]
pub enum GenericInstruction {
    /// Doesn't take any operand
    VOID(Instruction<()>),
    /// Takes 8-bit data operand                                        [d8]
    VALUE(Instruction<Value>),
    /// Takes 16-bit little-endian data                                 [d16]
    WIDE(Instruction<WideValue>),
    /// Takes 8-bit data (offset from $FF00)                            [a8]
    NEAR(Instruction<NearAddress>),
    /// Takes 16-bit little endian address                              [a16]
    FAR(Instruction<FarAddress>),
    /// Takes 8-bit signed data, effectively an offset for an address   [r8]
    OFFSET(Instruction<AddressOffset>),
}

impl<T> std::fmt::Debug for Instruction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("OP Code", &format_args!("{:#04X}", self.opcode))
            .field("ASM Name", &self.disassembly)
            .field("Byte size", &self.byte_size)
            .field("CPU Clock ticks", &self.clock_tick)
            .finish_non_exhaustive()
    }
}

impl<T> Instruction<T> {
    pub fn execute(&self, memory: &mut Memory, value: T) {
        log!("INSTRUCTION", format!("Executing {:?}", self));
        (self.function)(memory, value);
    }
}

pub fn instruction_from_opcode(opcode: OpCode) -> GenericInstruction {
    let opcode = opcode as usize;
    debug_assert!(opcode < INSTRUCTIONS.len());
    return INSTRUCTIONS[opcode];
}
