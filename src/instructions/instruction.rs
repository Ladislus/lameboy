use std::fmt;
use std::fmt::Debug;
use crate::instructions::instruction_set::{INSTRUCTION_INFORMATION, InstructionInformation};
use crate::memory::Memory;
use crate::log;

pub type OPCode = u8;
pub type InstructionFn = fn(&Instruction, &mut Memory);

pub struct Instruction {
    opcode: OPCode,
    name: String,
    length: usize,
    function: InstructionFn,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("OP Code", &self.opcode)
            .field("ASM Name", &self.name)
            .field("Length", &self.length)
            .finish()
    }
}

impl Instruction {
    fn from(info: &InstructionInformation) -> Instruction {
        return Instruction { opcode: info.0, name: String::from(info.1), length: info.2, function: info.3 }
    }

    #[cfg(debug_assertions)]
    fn log(&self) {
        log!("INSTRUCTION", format!("Running instruction: {:?}", self));
    }

    #[cfg(not(debug_assertions))]
    fn log(&self) {}

    pub fn get(opcode: OPCode) -> Instruction {
        assert!(opcode < (INSTRUCTION_INFORMATION.len() as u8));

        return Instruction::from(&INSTRUCTION_INFORMATION[opcode as usize]);
    }

    pub fn execute(&self, memory: &mut Memory) {
        self.log();
        (self.function)(self, memory)
    }
}
