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
    let wide_0 = simple.0 as WideType;
    let wide_1 = simple.1 as WideType;
    return (wide_0 << 8) + wide_1;
}

pub fn wide_to_simple(wide: WideType) -> SimpleType {

}