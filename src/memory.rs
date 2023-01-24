use crate::register::Registers;

pub struct Memory {
    pub size: usize,
    pub memory: Vec<u8>,
    pub mask: Vec<u8>,
    pub corrupted: bool,
    pub registers: Registers,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory { size, memory: vec![0; size], mask: vec![0; size], corrupted: false, registers: Registers::new() }
    }
}