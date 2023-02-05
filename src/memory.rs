use crate::register::{Registers, simple_to_wide, wide_to_simple};

const DATA_ADDR_START: usize = 0xFF00;

type Byte = u8;

pub type SimpleValue = Byte;
pub type WideValue = u16;

pub type RawMemory = Box<[Byte]>;

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

    pub fn set_memory_simple(&mut self, addr: usize, value: SimpleValue) {
        debug_assert!((DATA_ADDR_START + addr) < self.size);
        self.memory[DATA_ADDR_START + addr] = value;
    }

    pub fn read_memory_simple(&self, addr: usize) -> SimpleValue {
        debug_assert!((DATA_ADDR_START + addr) < self.size);
        return self.memory[DATA_ADDR_START + addr];
    }

    pub fn set_memory_wide(&mut self, addr: usize, value: WideValue) {
        debug_assert!((DATA_ADDR_START + addr) < self.size);
        // TODO: Check endianness
        let value = wide_to_simple(value);
        self.set_memory_simple(addr, value.0);
        self.set_memory_simple(addr + 1, value.1);
    }

    pub fn read_memory_wide(&self, addr: usize) -> WideValue {
        debug_assert!((DATA_ADDR_START + addr) < self.size);
        // TODO: Check endianness
        return simple_to_wide(&(self.memory[DATA_ADDR_START + addr], self.memory[DATA_ADDR_START + addr + 1]));
    }
}