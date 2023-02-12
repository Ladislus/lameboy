use crate::cpu::memory::Memory;
use crate::cpu::operations::INSTRUCTIONS;
use crate::utils::log::log;
use crate::utils::types::{FarAddress, NearAddress, AddressOffset, Value, Void, WideValue};

pub type OpCode = u8;
pub type InstructionFn<T> = fn(&Instruction<T>, &mut Memory, values: T);

// Instruction type aliases
pub type VoidInstruction = Instruction<Void>;
pub type ValueInstruction = Instruction<Value>;
pub type WideValueInstruction = Instruction<WideValue>;
pub type NearAddressInstruction = Instruction<NearAddress>;
pub type FarAddressInstruction = Instruction<FarAddress>;
pub type OffsetInstruction = Instruction<AddressOffset>;

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
    DATA8(Instruction<Value>),
    /// Takes 16-bit little-endian data
    DATA16(Instruction<WideValue>),
    /// Takes 8-bit data (offset for $FF00)
    ADDR8(Instruction<NearAddress>),
    /// Takes 16-bit little endian address
    ADDR16(Instruction<FarAddress>),
    /// Takes 8-bit signed data
    OFFSET(Instruction<AddressOffset>)
}

impl<T> std::fmt::Debug for Instruction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
