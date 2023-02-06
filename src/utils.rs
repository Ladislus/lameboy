use std::fmt::Display;
use crate::memory::{SimpleValue, WideValue};

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        // Because macro arguments don't have types, force type by assigning to variable which is typed
        let _p: &str = $prefix;
        println!("{:<40}\t{}", format!("{:<15} {}", format!("[{}]", $prefix), format!("({}:{})", file!(), line!())), $msg);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => ();
}

pub fn get_bit(old_value: SimpleValue, bit_index: usize) -> bool {
    let bit = ((old_value >> bit_index) & 1) != 0;

    // log!("UTILS", format!("get_bit {0:#0width$b}[{1}] ({0}) = {2}", old_value, bit_index, bit as u8, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return bit;
}

pub fn set_bit(old_value: SimpleValue, bit_index: usize) -> SimpleValue {
    let new_value = old_value | (1 << bit_index);

    // log!("UTILS", format!("set_bit {0:#0width$b}[{2}] ({0}) => {1:#0width$b} ({1})", old_value, new_value, bit_index, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return new_value;
}

pub fn clear_bit(old_value: SimpleValue, bit_index: usize) -> SimpleValue {
    let new_value = old_value & !(1 << bit_index);

    // log!("UTILS", format!("clear_bit {0:#0width$b}[{2}] ({0}) => {1:#0width$b} ({1})", old_value, new_value, bit_index, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return new_value;
}

pub fn assign_bit(old_value: SimpleValue, bit_index: usize, status: bool) -> SimpleValue {
    let new_value = (old_value & !(1 << bit_index)) | ((status as SimpleValue) << bit_index);

    // log!("UTILS", format!("assign_bit {0:#0width$b}[{2}] ({0}) = {3} => {1:#0width$b} ({1})", old_value, new_value, bit_index, status as u8, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return new_value;
}

fn check_half_carry_add<T: Into<usize> + Display + std::fmt::Binary + Copy>(old_value: T, value: T, index: usize) -> bool {
    debug_assert!(index < ((core::mem::size_of::<T>() * 8) - 1));
    let test_mask: usize = 1 << (index + 1);
    let and_mask: usize = test_mask - 1;
    let result = ((old_value.into() & and_mask) + (value.into() & and_mask) & test_mask) != 0;

    log!("UTILS", format!("{0} + {1} ({0:#0width$b} + {1:#0width$b}) would carry on bit {2} ? {3}", old_value, value, index, result, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return result;
}

fn check_half_carry_sub<T: Into<usize> + Display + std::fmt::Binary + Copy>(old_value: T, value: T, index: usize) -> bool {
    debug_assert!(index < ((core::mem::size_of::<T>() * 8) - 1));
    let test_mask: usize = 1 << (index + 1);
    let and_mask: usize = test_mask - 1;
    let result = ((old_value.into() & and_mask).wrapping_sub(value.into() & and_mask) & test_mask) != 0;

    log!("UTILS", format!("{0} + {1} ({0:#0width$b} - {1:#0width$b}) would carry on bit {2} ? {3}", old_value, value, index, result, width = core::mem::size_of::<SimpleValue>() * 8 + 1));

    return result;
}

pub fn check_half_carry_simple_add(old_value: SimpleValue, value: SimpleValue) -> bool {
    return check_half_carry_add(old_value, value, 3);
}

pub fn check_half_carry_simple_sub(old_value: SimpleValue, value: SimpleValue) -> bool {
    return check_half_carry_sub(old_value, value, 3)
}

pub fn check_half_carry_wide_add(old_value: WideValue, value: WideValue) -> bool {
    return check_half_carry_add(old_value, value, 11);
}

pub fn check_half_carry_wide_sub(old_value: WideValue, value: WideValue) -> bool {
    return check_half_carry_sub(old_value, value, 11)
}

// TODO Add tests