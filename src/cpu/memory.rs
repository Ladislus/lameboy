use crate::cpu::register::RegisterGroup;
use crate::utils::bits::bit_size;
use crate::utils::conversions::{pair_to_wide, wide_to_pair};
use crate::utils::log::log;
use crate::utils::types::{Byte, FarAddress, NearAddress, Value, WideValue};

const NEAR_ADDR_START: usize = 0xFF00;

type MemoryPtr = Box<[Byte]>;
type Stack = Vec<Value>;

pub struct Memory {
    pub size: usize,
    pub memory: MemoryPtr,
    pub stack: Stack,
    pub registers: RegisterGroup,
}

impl Memory {
    pub fn new(size: usize) -> Memory {

        log!("MEMORY", format!("Creating {} bytes memory", size));

        Memory {
            size,
            memory: vec![0; size].into_boxed_slice(),
            stack: Stack::new(),
            registers: RegisterGroup::new()
        }
    }

    fn near_to_far(addr: NearAddress) -> usize {
        let far_addr = (addr as usize) + NEAR_ADDR_START;
        log!("MEMORY", format!("Near Address {:#x} + {:#x} = {:#x}", addr, NEAR_ADDR_START, far_addr));
        return far_addr;
    }

    // TODO: manage stack size
    pub fn stack_push_wide(&mut self, value: WideValue) {

        let splitted = wide_to_pair(value);

        self.stack[(self.registers.SP - 1) as usize] = splitted.0;
        self.stack[(self.registers.SP - 2) as usize] = splitted.1;

        // Comes from:
        // http://gameboy.mongenel.com/dmg/opcodes.html
        //      (SP-1) <-  HIGH(Value), (SP-2) <-  LOW(value), SP <- SP-2
        self.registers.SP -= 2;
    }

    // TODO: manage stack size
    pub fn stack_pop_wide(&mut self) -> WideValue {
        let low = self.stack[self.registers.SP as usize] as WideValue;
        let high = self.stack[(self.registers.SP + 1) as usize] as WideValue;
        let value = (high << (bit_size(high) / 2)) + low;

        // Comes from:
        // >    ld LOW(r16), [sp] ; C, E or L
        // >    inc sp
        // >    ld HIGH(r16), [sp] ; B, D or H
        // >    inc sp
        // So SP is incremented 2 times
        // or
        // http://gameboy.mongenel.com/dmg/opcodes.html
        // >    	LOW(register) <- (SP), HIGH(register) <- (SP+1), SP <- SP+2
        self.registers.SP += 2;

        return value;
    }

    pub fn write_near_addr(&mut self, addr: NearAddress, value: Value) {
        let addr = Self::near_to_far(addr);
        debug_assert!(addr < self.size);
        log!("MEMORY", format!("Write {:#x} at address ${:#x}", value, addr));
        self.memory[addr] = value;
    }

    pub fn write_far_addr(&mut self, addr: FarAddress, value: Value) {
        let addr = addr as usize;
        debug_assert!(addr< self.size);
        log!("MEMORY", format!("Write {:#x} at address ${:#x}", value, addr));
        self.memory[addr] = value;
    }

    pub fn read_near_addr(&self, addr: NearAddress) -> Value {
        let addr = Self::near_to_far(addr);
        debug_assert!(addr < self.size);
        let read = self.memory[addr];
        log!("MEMORY", format!("Read {:#x} at address ${:#x}", read, addr));
        return read;
    }

    pub fn read_far_addr(&self, addr: FarAddress) -> Value {
        let addr = addr as usize;
        debug_assert!(addr < self.size);
        let read = self.memory[addr];
        log!("MEMORY", format!("Read {:#x} at address ${:#x}", read, addr));
        return read;
    }

    // TODO: Check endianness
    pub fn write_wide_near_addr(&mut self, addr: NearAddress, value: WideValue) {
        let addr = Self::near_to_far(addr);
        debug_assert!((addr + 1) < self.size);
        log!("MEMORY", format!("Write {:#x} at address ${:#x}", value, addr));
        let values = wide_to_pair(value);
        self.memory[addr] = values.0;
        self.memory[addr + 1] = values.1
    }

    // TODO: Check endianness
    pub fn write_wide_far_addr(&mut self, addr: FarAddress, value: WideValue) {
        let addr = addr as usize;
        debug_assert!((addr + 1) < self.size);
        log!("MEMORY", format!("Write {:#x} at address ${:#x}", value, addr));
        let values = wide_to_pair(value);
        self.memory[addr] = values.0;
        self.memory[addr + 1] = values.1
    }

    // TODO: Check endianness
    pub fn read_wide_near_addr(&self, addr: NearAddress) -> WideValue {
        let addr = Self::near_to_far(addr);
        debug_assert!((addr + 1) < self.size);
        let read = pair_to_wide(&(self.memory[addr], self.memory[addr + 1]));
        log!("MEMORY", format!("Read {:#x} at address ${:#x}", read, addr));
        return read;
    }

    // TODO: Check endianness
    pub fn read_wide_far_addr(&self, addr: FarAddress) -> WideValue {
        let addr = addr as usize;
        debug_assert!((addr + 1) < self.size);
        let read = pair_to_wide(&(self.memory[addr], self.memory[addr + 1]));
        log!("MEMORY", format!("Read {:#x} at address ${:#x}", read, addr));
        return read;
    }
}

// TODO: Add tests