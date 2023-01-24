pub type WideType = u16;
pub type SimpleType = u8;

pub union Register {
    pub wide: WideType,
    pub simple: (SimpleType, SimpleType)
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
        Registers { AF: Register { wide: 0 }, BC: Register { wide: 0 }, DE: Register { wide: 0 }, HL: Register { wide: 0 }, SP: 0, PC: 0 }
    }
}