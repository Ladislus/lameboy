pub type WideType = u16;
pub type SimpleType = (u8, u8);

pub union Register {
    pub wide: WideType,
    pub simple: SimpleType
}

#[allow(non_snake_case)]
pub struct Registers {
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,

    pub SP: u16,
    pub PC: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            AF: Register { simple: (0, 0) },
            BC: Register { simple: (0, 0) },
            DE: Register { simple: (0, 0) },
            HL: Register { simple: (0, 0) },

            SP: 0,
            PC: 0
        }
    }
}

pub fn simple_to_wide(simple: &SimpleType) -> WideType {
    return ((simple.0 as WideType) << 8) + (simple.1 as WideType);
}

pub fn wide_to_simple(wide: WideType) -> SimpleType {
    return ((wide >> 8) as u8, wide as u8);
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