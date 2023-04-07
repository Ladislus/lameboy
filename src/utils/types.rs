pub type Byte = u8;

pub type Void = ();

pub type Value = u8;
pub type WideValue = u16;

pub type NearAddress = u8;
pub type FarAddress = u16;
pub type AddressOffset = i8;

pub type PairRegister = (Value, Value);
pub type WideRegister = WideValue;

#[cfg(test)]
mod tests {
    use super::{Byte, Value, WideValue};

    #[test]
    fn test_type_aliases() {
        assert_eq!(std::mem::size_of::<Value>() * 2, std::mem::size_of::<WideValue>());
        assert_eq!(std::mem::size_of::<Byte>(), std::mem::size_of::<Value>());
    }
}