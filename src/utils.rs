use crate::memory::{SimpleValue, WideValue};

#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        // Because macro arguments don't have types, force type by assigning to variable which is typed
        let _p: &str = $prefix;
        println!("{:<40}\t{}", format!("{:<15} {}", format!("[{}]", $prefix), format!("({}:{})", file!(), line!())), $msg);
    };
}

pub fn get_bit(value: SimpleValue, index: usize) -> bool {
    return ((value >> index) & 1) != 0;
}

pub fn set_bit(value: SimpleValue, index: usize) -> SimpleValue {
    return value | (1 << index);
}

pub fn clear_bit(value: SimpleValue, index: usize) -> SimpleValue {
    return value & !(1 << index);
}

pub fn assign_bit(value: SimpleValue, index: usize, status: bool) -> SimpleValue {
    return (value & !(1 << index)) | ((status as SimpleValue) << index)
}

fn check_half_carry_add<T>(old_value: usize, value: usize, index: usize) -> bool {
    debug_assert!(index < ((core::mem::size_of::<T>() * 8) - 1));
    let test = 1 << (index + 1);
    let mask = test - 1;
    return ((old_value & mask) + (value & mask) & test) != 0;
}

fn check_half_carry_sub<T>(old_value: usize, value: usize, index: usize) -> bool {
    debug_assert!(index < ((core::mem::size_of::<T>() * 8) - 1));
    let test: usize = 1 << (index + 1);
    let mask: usize = test - 1;
    return ((old_value & mask).wrapping_sub(value & mask) & test) != 0;
}

pub fn check_half_carry_simple_add(old_value: SimpleValue, value: SimpleValue) -> bool {
    return check_half_carry_add::<SimpleValue>(old_value as usize, value as usize, 3);
}

pub fn check_half_carry_simple_sub(old_value: SimpleValue, value: SimpleValue) -> bool {
    return check_half_carry_sub::<SimpleValue>(old_value as usize, value as usize, 3)
}

pub fn check_half_carry_wide_add(old_value: WideValue, value: WideValue) -> bool {
    return check_half_carry_add::<WideValue>(old_value as usize, value as usize, 11);
}

pub fn check_half_carry_wide_sub(old_value: WideValue, value: WideValue) -> bool {
    return check_half_carry_sub::<WideValue>(old_value as usize, value as usize, 11)
}