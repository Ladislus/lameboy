use crate::utils::types::{PairRegister, Value, WideRegister, WideValue};

// TODO: Check endianness
pub fn pair_to_wide(pair: &PairRegister) -> WideRegister {
    return ((pair.0 as WideValue) << 8) + (pair.1 as WideValue);
}

// TODO: Check endianness
pub fn wide_to_pair(wide: WideRegister) -> PairRegister {
    return ((wide >> 8) as Value, wide as Value);
}

#[cfg(test)]
mod tests {
    use crate::utils::conversions::{pair_to_wide, wide_to_pair};

    #[test]
    fn test_wide_to_simple() {
        assert_eq!(wide_to_pair(0b1111_1111_1111_1111), (0b1111_1111, 0b1111_1111));
        assert_eq!(wide_to_pair(0b0000_0000_0000_0000), (0b0000_0000, 0b0000_0000));
        assert_eq!(wide_to_pair(0b1110_1111_1111_0111), (0b1110_1111, 0b1111_0111));
        assert_eq!(wide_to_pair(0b1000_0000_0000_0111), (0b1000_0000, 0b0000_0111));
    }

    #[test]
    fn test_simple_to_wide() {
        assert_eq!(pair_to_wide(&(0b1111_1111, 0b1111_1111)), 0b1111_1111_1111_1111);
        assert_eq!(pair_to_wide(&(0b0000_0000, 0b0000_0000)), 0b0000_0000_0000_0000);
        assert_eq!(pair_to_wide(&(0b1111_0000, 0b0000_1111)), 0b1111_0000_0000_1111);
        assert_eq!(pair_to_wide(&(0b1000_0000, 0b0000_0001)), 0b1000_0000_0000_0001);
        assert_eq!(pair_to_wide(&(0b0000_0000, 0b0000_0001)), 0b0000_0000_0000_0001);
    }
}