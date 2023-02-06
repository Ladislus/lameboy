use crate::register::{Registers, simple_to_wide, wide_to_simple};

const SIMPLE_ADDR_START: usize = 0xFF00;

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

    pub fn write_simple_near_addr(&mut self, addr: SimpleValue, value: SimpleValue) {
        let addr = addr as usize;
        debug_assert!((SIMPLE_ADDR_START + addr) < self.size);
        self.memory[SIMPLE_ADDR_START + addr] = value;
    }

    pub fn write_simple_far_addr(&mut self, addr: WideValue, value: SimpleValue) {
        let addr = addr as usize;
        debug_assert!(addr< self.size);
        self.memory[addr] = value;
    }

    pub fn read_simple_near_addr(&self, addr: SimpleValue) -> SimpleValue {
        let addr = addr as usize;
        debug_assert!((SIMPLE_ADDR_START + addr) < self.size);
        return self.memory[SIMPLE_ADDR_START + addr];
    }

    pub fn read_simple_far_addr(&self, addr: WideValue) -> SimpleValue {
        let addr = addr as usize;
        debug_assert!(addr < self.size);
        return self.memory[addr];
    }

    // TODO: Check endianness
    pub fn write_wide_near_addr(&mut self, addr: SimpleValue, value: WideValue) {
        let addr = addr as usize;
        debug_assert!((SIMPLE_ADDR_START + addr + 1) < self.size);
        let value = wide_to_simple(value);
        self.memory[SIMPLE_ADDR_START + addr] = value.0;
        self.memory[SIMPLE_ADDR_START + addr + 1] = value.1
    }

    // TODO: Check endianness
    pub fn write_wide_far_addr(&mut self, addr: WideValue, value: WideValue) {
        let addr = addr as usize;
        debug_assert!(addr + 1 < self.size);
        let value = wide_to_simple(value);
        self.memory[addr] = value.0;
        self.memory[addr + 1] = value.1
    }

    // TODO: Check endianness
    pub fn read_wide_near_addr(&self, addr: SimpleValue) -> WideValue {
        let addr = addr as usize;
        debug_assert!((SIMPLE_ADDR_START + addr + 1) < self.size);
        return simple_to_wide(&(self.memory[SIMPLE_ADDR_START + addr], self.memory[SIMPLE_ADDR_START + addr + 1]));
    }

    // TODO: Check endianness
    pub fn read_wide_far_addr(&self, addr: WideValue) -> WideValue {
        let addr = addr as usize;
        debug_assert!(addr + 1 < self.size);
        return simple_to_wide(&(self.memory[addr], self.memory[addr + 1]));

    }
}