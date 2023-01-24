use crate::instructions::instruction::{InstructionFn, OPCode};
use crate::instructions::instruction_functions::{noop, unimplemented};

pub struct InstructionInformation(pub OPCode, pub &'static str, pub usize, pub InstructionFn);

// TODO: Fill all instruction names/opcodes, defaulting function to unimplemented
pub static INSTRUCTION_INFORMATION: [InstructionInformation; 2] = [
    InstructionInformation(0x00, "NOOP", 0, noop),
    InstructionInformation(0x01, "", 0, unimplemented)
];