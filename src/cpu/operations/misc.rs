use crate::cpu::memory::Memory;
use crate::utils::types::Void;

pub fn none(_memory: &mut Memory, _value: Void) {
    panic!("Instruction doesn't exist");
}

pub fn noop(_memory: &mut Memory, _value: Void) {}

pub fn prefix(_memory: &mut Memory, _value: Void) {
    panic!("Function for instruction 0xCB should never be called, as it's a prefix OP code");
}

pub fn halt(_memory: &mut Memory, _value: Void) {
    // https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/#HALT
    todo!("Enter CPU low-power consumption mode until an interrupt occurs. The exact behavior of this instruction depends on the state of the IME flag.")
}

pub fn stop(_memory: &mut Memory, _value: u8) {
    // https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/#STOP
    todo!("Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.")
}