use crate::memory::{SimpleValue, WideValue};

pub type SimpleRegister = (SimpleValue, SimpleValue);
pub type WideRegister = WideValue;

pub union Register {
    pub as_wide: WideRegister,
    pub as_two_simple: SimpleRegister
}

#[allow(non_snake_case)]
pub struct Registers {
    AF: Register,
    BC: Register,
    DE: Register,
    HL: Register,

    SP: u16,
    PC: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            AF: Register { as_two_simple: (0, 0) },
            BC: Register { as_two_simple: (0, 0) },
            DE: Register { as_two_simple: (0, 0) },
            HL: Register { as_two_simple: (0, 0) },

            SP: 0,
            PC: 0
        }
    }

    pub fn get_af(&self) -> WideValue { unsafe { return self.AF.as_wide; } }
    pub fn set_af(&mut self, value: WideValue) { self.AF.as_wide = value; }
    pub fn get_a(&self) -> SimpleValue { unsafe { return self.AF.as_two_simple.0; } }
    pub fn set_a(&mut self, value: SimpleValue) { self.AF.as_two_simple.0 = value; }
    pub fn get_f(&self) -> SimpleValue { unsafe { return self.AF.as_two_simple.1; } }
    pub fn set_f(&mut self, value: SimpleValue) { self.AF.as_two_simple.1 = value; }

    pub fn get_bc(&self) -> WideValue { unsafe { return self.BC.as_wide; } }
    pub fn set_bc(&mut self, value: WideValue) { self.BC.as_wide = value; }
    pub fn get_b(&self) -> SimpleValue { unsafe { return self.BC.as_two_simple.0; } }
    pub fn set_b(&mut self, value: SimpleValue) { self.BC.as_two_simple.0 = value; }
    pub fn get_c(&self) -> SimpleValue { unsafe { return self.BC.as_two_simple.1; } }
    pub fn set_c(&mut self, value: SimpleValue) { self.BC.as_two_simple.1 = value; }

    pub fn get_de(&self) -> WideValue { unsafe { return self.DE.as_wide; } }
    pub fn set_de(&mut self, value: WideValue) { self.DE.as_wide = value; }
    pub fn get_d(&self) -> SimpleValue { unsafe { return self.DE.as_two_simple.0; } }
    pub fn set_d(&mut self, value: SimpleValue) { self.DE.as_two_simple.0 = value; }
    pub fn get_e(&self) -> SimpleValue { unsafe { return self.DE.as_two_simple.1; } }
    pub fn set_e(&mut self, value: SimpleValue) { self.DE.as_two_simple.1 = value; }

    pub fn get_hl(&self) -> WideValue { unsafe { return self.HL.as_wide; } }
    pub fn set_hl(&mut self, value: WideValue) { self.HL.as_wide = value; }
    pub fn get_h(&self) -> SimpleValue { unsafe { return self.HL.as_two_simple.0; } }
    pub fn set_h(&mut self, value: SimpleValue) { self.HL.as_two_simple.0 = value; }
    pub fn get_l(&self) -> SimpleValue { unsafe { return self.HL.as_two_simple.1; } }
    pub fn set_l(&mut self, value: SimpleValue) { self.HL.as_two_simple.1 = value; }
}

pub fn simple_to_wide(simple: &SimpleRegister) -> WideRegister {
    return ((simple.0 as WideValue) << 8) + (simple.1 as WideValue);
}

pub fn wide_to_simple(wide_register: WideRegister) -> SimpleRegister {
    return ((wide_register >> 8) as SimpleValue, wide_register as SimpleValue);
}

#[cfg(test)]
mod tests {
    use crate::register::{simple_to_wide, wide_to_simple};

    #[test]
    fn test_wide_to_simple() {
        assert_eq!(wide_to_simple(0b1111_1111_1111_1111), (0b1111_1111, 0b1111_1111));
        assert_eq!(wide_to_simple(0b0000_0000_0000_0000), (0b0000_0000, 0b0000_0000));
        assert_eq!(wide_to_simple(0b1110_1111_1111_0111), (0b1110_1111, 0b1111_0111));
        assert_eq!(wide_to_simple(0b1000_0000_0000_0111), (0b1000_0000, 0b0000_0111));
    }

    #[test]
    fn test_simple_to_wide() {
        assert_eq!(simple_to_wide(&(0b1111_1111, 0b1111_1111)), 0b1111_1111_1111_1111);
        assert_eq!(simple_to_wide(&(0b0000_0000, 0b0000_0000)), 0b0000_0000_0000_0000);
        assert_eq!(simple_to_wide(&(0b1111_0000, 0b0000_1111)), 0b1111_0000_0000_1111);
        assert_eq!(simple_to_wide(&(0b1000_0000, 0b0000_0001)), 0b1000_0000_0000_0001);
        assert_eq!(simple_to_wide(&(0b0000_0000, 0b0000_0001)), 0b0000_0000_0000_0001);
    }
}