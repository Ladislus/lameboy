use crate::cpu::register::RegisterGroup;
use crate::cpu::stack::Stack;
use crate::utils::conversions::wide_to_pair;
use crate::utils::log::log;
use crate::utils::types::{Byte, FarAddress, NearAddress, Value, WideValue};

const NEAR_ADDR_START: usize = 0xFF00;

type MemoryPtr = Box<[Byte]>;

pub struct Memory {
    pub size: usize,
    pub memory: MemoryPtr,
    pub stack: Stack,
    pub registers: RegisterGroup,
}

impl Memory {
    pub fn new(size: usize) -> Memory {

        log!("MEMORY", format!("Creating {size} bytes memory"));

        Memory {
            size,
            memory: vec![0; size].into_boxed_slice(),
            stack: Stack::new(u16::MAX),
            registers: RegisterGroup::new()
        }
    }

    fn near_to_far(addr: NearAddress) -> usize {
        let far_addr = (addr as usize) + NEAR_ADDR_START;

        log!("MEMORY", format!("Near Address {addr:#x} + {NEAR_ADDR_START:#x} = {far_addr:#x}"));

        return far_addr;
    }

    pub fn write_near_addr(&mut self, addr: NearAddress, value: Value) {
        let addr = Self::near_to_far(addr);
        debug_assert!(addr < self.size);

        log!("MEMORY", format!("Write {value:#x} at address ${addr:#x}"));

        self.memory[addr] = value;
    }

    pub fn write_far_addr(&mut self, addr: FarAddress, value: Value) {
        let addr = addr as usize;
        debug_assert!(addr< self.size);

        log!("MEMORY", format!("Write {value:#x} at address ${addr:#x}"));

        self.memory[addr] = value;
    }

    pub fn read_near_addr(&self, addr: NearAddress) -> Value {
        let addr = Self::near_to_far(addr);
        debug_assert!(addr < self.size);

        let read = self.memory[addr];

        log!("MEMORY", format!("Read {read:#x} at address ${addr:#x}"));

        return read;
    }

    pub fn read_far_addr(&self, addr: FarAddress) -> Value {
        let addr = addr as usize;
        debug_assert!(addr < self.size);

        let read = self.memory[addr];

        log!("MEMORY", format!("Read {read:#x} at address ${addr:#x}"));

        return read;
    }

    // TODO: Check endianness
    // pub fn write_wide_near_addr(&mut self, addr: NearAddress, value: WideValue) {
    //     let addr = Self::near_to_far(addr);
    //     debug_assert!((addr + 1) < self.size);
    //
    //     log!("MEMORY", format!("Write {value:#x} at address ${addr:#x}"));
    //
    //     let values = wide_to_pair(value);
    //
    //     self.memory[addr] = values.0;
    //     self.memory[addr + 1] = values.1
    // }

    // TODO: Check endianness
    pub fn write_wide_far_addr(&mut self, addr: FarAddress, value: WideValue) {
        let addr = addr as usize;
        debug_assert!((addr + 1) < self.size);

        log!("MEMORY", format!("Write {value:#x} at address ${addr:#x}"));

        let values = wide_to_pair(value);

        self.memory[addr] = values.0;
        self.memory[addr + 1] = values.1;
    }

    // TODO: Check endianness
    // pub fn read_wide_near_addr(&self, addr: NearAddress) -> WideValue {
    //     let addr = Self::near_to_far(addr);
    //     debug_assert!((addr + 1) < self.size);
    //
    //     let read = pair_to_wide(self.memory[addr], self.memory[addr + 1]);
    //
    //     log!("MEMORY", format!("Read {read:#x} at address ${addr:#x}"));
    //
    //     return read;
    // }

    // TODO: Check endianness
    // pub fn read_wide_far_addr(&self, addr: FarAddress) -> WideValue {
    //     let addr = addr as usize;
    //     debug_assert!((addr + 1) < self.size);
    //
    //     let read = pair_to_wide(self.memory[addr], self.memory[addr + 1]);
    //
    //     log!("MEMORY", format!("Read {read:#x} at address ${addr:#x}"));

    //     return read;
    // }
}

// TODO: Add tests