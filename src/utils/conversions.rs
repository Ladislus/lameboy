use crate::utils::types::{AddressOffset, FarAddress, PairRegister, Value, WideRegister, WideValue};

// TODO: Check endianness
pub fn pair_to_wide(high: Value, low: Value) -> WideRegister {
    return WideValue::from_be_bytes([high, low]);
}

// TODO: Check endianness
pub fn wide_to_pair(wide: WideRegister) -> PairRegister {
    let bytes: [u8; 2] = wide.to_be_bytes();
    return (bytes[0], bytes[1]);
}

#[allow(clippy::cast_sign_loss)]
pub fn offset_to_far_address(offset: AddressOffset) -> FarAddress {
    return offset as FarAddress;
}

#[cfg(test)]
mod tests {
    use crate::utils::types::{AddressOffset, FarAddress};
    use super::{pair_to_wide, wide_to_pair, offset_to_far_address};

    #[test]
    fn test_wide_to_simple() {
        assert_eq!(wide_to_pair(0b1111_1111_1111_1111), (0b1111_1111, 0b1111_1111));
        assert_eq!(wide_to_pair(0b0000_0000_0000_0000), (0b0000_0000, 0b0000_0000));
        assert_eq!(wide_to_pair(0b1110_1111_1111_0111), (0b1110_1111, 0b1111_0111));
        assert_eq!(wide_to_pair(0b1000_0000_0000_0111), (0b1000_0000, 0b0000_0111));
    }

    #[test]
    fn test_simple_to_wide() {
        assert_eq!(pair_to_wide(0b1111_1111, 0b1111_1111), 0b1111_1111_1111_1111);
        assert_eq!(pair_to_wide(0b0000_0000, 0b0000_0000), 0b0000_0000_0000_0000);
        assert_eq!(pair_to_wide(0b1111_0000, 0b0000_1111), 0b1111_0000_0000_1111);
        assert_eq!(pair_to_wide(0b1000_0000, 0b0000_0001), 0b1000_0000_0000_0001);
        assert_eq!(pair_to_wide(0b0000_0000, 0b0000_0001), 0b0000_0000_0000_0001);
    }

    #[test]
    fn test_offset_to_far_address() {
        assert_eq!(offset_to_far_address(-1), FarAddress::MAX);
        assert_eq!(offset_to_far_address(1), 1);
        assert_eq!(offset_to_far_address(AddressOffset::MAX), AddressOffset::MAX as FarAddress);
    }
}