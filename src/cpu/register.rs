use crate::utils::bits::{assign_bit, get_bit};
use crate::utils::types::{FarAddress, PairRegister, Value, WideRegister, WideValue};

pub union Register {
    pub as_wide: WideRegister,
    pub as_pair: PairRegister
}

const ZERO_FLAG_OFFSET: usize = 7;
const SUBTRACTION_FLAG_OFFSET: usize = 6;
const HALF_CARRY_FLAG_OFFSET: usize = 5;
const CARRY_FLAG_OFFSET: usize = 4;

const CLEAR_MASK: Value = 0b1111_0000;

#[allow(non_snake_case)]
pub struct RegisterGroup {
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,

    pub SP: FarAddress,
    pub PC: FarAddress,
}

impl RegisterGroup {
    pub fn new() -> RegisterGroup {
        RegisterGroup {
            AF: Register { as_pair: (0, 0) },
            BC: Register { as_pair: (0, 0) },
            DE: Register { as_pair: (0, 0) },
            HL: Register { as_pair: (0, 0) },

            SP: 0,
            PC: 0
        }
    }

    pub fn get_af(&self) -> WideValue { unsafe { return self.AF.as_wide; } }
    pub fn set_af(&mut self, value: WideValue) { self.AF.as_wide = value; }
    pub fn get_a(&self) -> Value { unsafe { return self.AF.as_pair.0; } }
    pub fn set_a(&mut self, value: Value) { self.AF.as_pair.0 = value; }
    pub fn get_f(&self) -> Value { unsafe { return self.AF.as_pair.1; } }
    pub fn set_f(&mut self, value: Value) { self.AF.as_pair.1 = value; }

    pub fn get_bc(&self) -> WideValue { unsafe { return self.BC.as_wide; } }
    pub fn set_bc(&mut self, value: WideValue) { self.BC.as_wide = value; }
    pub fn get_b(&self) -> Value { unsafe { return self.BC.as_pair.0; } }
    pub fn set_b(&mut self, value: Value) { self.BC.as_pair.0 = value; }
    pub fn get_c(&self) -> Value { unsafe { return self.BC.as_pair.1; } }
    pub fn set_c(&mut self, value: Value) { self.BC.as_pair.1 = value; }

    pub fn get_de(&self) -> WideValue { unsafe { return self.DE.as_wide; } }
    pub fn set_de(&mut self, value: WideValue) { self.DE.as_wide = value; }
    pub fn get_d(&self) -> Value { unsafe { return self.DE.as_pair.0; } }
    pub fn set_d(&mut self, value: Value) { self.DE.as_pair.0 = value; }
    pub fn get_e(&self) -> Value { unsafe { return self.DE.as_pair.1; } }
    pub fn set_e(&mut self, value: Value) { self.DE.as_pair.1 = value; }

    pub fn get_hl(&self) -> WideValue { unsafe { return self.HL.as_wide; } }
    pub fn set_hl(&mut self, value: WideValue) { self.HL.as_wide = value; }
    pub fn get_h(&self) -> Value { unsafe { return self.HL.as_pair.0; } }
    pub fn set_h(&mut self, value: Value) { self.HL.as_pair.0 = value; }
    pub fn get_l(&self) -> Value { unsafe { return self.HL.as_pair.1; } }
    pub fn set_l(&mut self, value: Value) { self.HL.as_pair.1 = value; }

    pub fn get_zero_flag(&self) -> bool { return get_bit(self.get_f(), ZERO_FLAG_OFFSET); }
    pub fn set_zero_flag(&mut self, status: bool) { self.set_f(assign_bit(self.get_f(), ZERO_FLAG_OFFSET, status) & CLEAR_MASK); }

    pub fn get_subtraction_flag(&self) -> bool { return get_bit(self.get_f(), SUBTRACTION_FLAG_OFFSET); }
    pub fn set_subtraction_flag(&mut self, status: bool) { self.set_f(assign_bit(self.get_f(), SUBTRACTION_FLAG_OFFSET, status) & CLEAR_MASK); }

    pub fn get_half_carry_flag(&self) -> bool { return get_bit(self.get_f(), HALF_CARRY_FLAG_OFFSET); }
    pub fn set_half_carry_flag(&mut self, status: bool) { self.set_f(assign_bit(self.get_f(), HALF_CARRY_FLAG_OFFSET, status) & CLEAR_MASK); }

    pub fn get_carry_flag(&self) -> bool { return get_bit(self.get_f(), CARRY_FLAG_OFFSET); }
    pub fn set_carry_flag(&mut self, status: bool) { self.set_f(assign_bit(self.get_f(), CARRY_FLAG_OFFSET, status) & CLEAR_MASK); }
}