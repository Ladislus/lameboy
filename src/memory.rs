use crate::register::Registers;

type Byte = u8;
type RawMemory = Box<[Byte]>;

pub struct Memory {
    pub size: usize,
    pub memory: RawMemory,
    pub mask: RawMemory,
    pub corrupted: bool,
    pub registers: Registers,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            size,
            memory: vec![0; size].into_boxed_slice(),
            mask: vec![0; size].into_boxed_slice(),
            corrupted: false,
            registers: Registers::new()
        }
    }
}