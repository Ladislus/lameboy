use std::ops::Sub;
use crate::log;
use crate::utils::traits::Integer;
use crate::utils::types::{Value, WideValue};

pub fn bit_size<T>(_: T) -> usize {
    return core::mem::size_of::<T>() * 8;
}

pub fn max_bit_index<T>(t: T) -> usize { return bit_size(t) - 1; }

pub fn get_bit<T: Integer + std::ops::Shr<usize, Output = T>>(value: T, bit_index: usize) -> bool {
    debug_assert!(bit_index < bit_size(value));

    let bit = ((value >> bit_index) & 1.into()) != 0.into();
    // log!("UTILS", format!("{0:#0width$b}[{1}] ({0}) = {2}", value, bit_index, bit as u8, width = bit_size(value) + 2));
    return bit;
}

pub fn set_bit<T: Integer + std::ops::BitOr<usize, Output = T>>(old_value: T, bit_index: usize) -> T {
    debug_assert!(bit_index < bit_size(old_value));

    let new_value = old_value | (1 << bit_index);
    // log!("UTILS", format!("{0:#0width$b}[{2}] ({0}) => {1:#0width$b} ({1})", old_value, new_value, bit_index, width = bit_size(old_value) + 2));
    return new_value;
}

pub fn clear_bit<T: Integer + std::ops::BitAnd<usize, Output = T>>(old_value: Value, bit_index: usize) -> Value {
    debug_assert!(bit_index < bit_size(old_value));

    let new_value = old_value & !(1 << bit_index);
    // log!("UTILS", format!("{0:#0width$b}[{2}] ({0}) => {1:#0width$b} ({1})", old_value, new_value, bit_index,  width = bit_size(old_value) + 2));
    return new_value;
}

pub fn assign_bit<T: Integer + std::ops::Shl<usize, Output = T>>(old_value: T, bit_index: usize, status: bool) -> T {
    debug_assert!(bit_index < bit_size(old_value));

    let new_value = (old_value & !(<u8 as Into<T>>::into(1) << bit_index)) | (<u8 as Into<T>>::into(status as u8) << bit_index);
    // log!("UTILS", format!("{0:#0width$b}[{2}] ({0}) = {3} => {1:#0width$b} ({1})", old_value, new_value, bit_index, status as u8, width = bit_size(old_value) + 2));
    return new_value;
}

fn check_carry_add<T: Integer + std::ops::Shl<usize, Output = T>>(value: T, operand: T, index: usize) -> bool {
    debug_assert!(index < (bit_size(value) - 1));

    let test_mask: T = <u8 as Into<T>>::into(1) << (index + 1);
    let and_mask: T = test_mask - 1.into();
    let result = ((value & and_mask) + (operand & and_mask) & test_mask) != 0.into();

    // log!("UTILS", format!("{0} + {1} ({0:#0width$b} + {1:#0width$b}) would carry on bit {2} ? {3}", value, operand, index, result, width = bit_size(value) + 2));

    return result;
}

fn check_carry_sub<T: Integer + std::ops::Shl<usize, Output = T>>(value: T, operand: T, bit_index: usize) -> bool
    where std::num::Wrapping<T>: Sub<Output = std::num::Wrapping<T>> {
    debug_assert!(bit_index < (bit_size(value) - 1));

    let test_mask: T = <u8 as Into<T>>::into(1) << (bit_index + 1);
    let and_mask: T = test_mask - 1.into();
    // Because "wrapping_sub" is not part of a trait but integer type directly, we can't certify that T has it (even though T is always an integer in reality, the type system can't be sure).
    // So instead, use std::num::Wrapping struct to act as a proxy which allow to use wrapping operations as default ones (e.g operator '-' becomes a wrapping_sub, operator '+' becomes a wrapping_add, ...)
    // on a restricted set of types, which we ensure by the fact that Wrapping<T> implements std::ops::Sub (e.i "impl Sub<Wrapping<T>> for Wrapping<T>", which is true for all integer types)
    // https://doc.rust-lang.org/stable/std/num/struct.Wrapping.html
    // let result = ((value & and_mask).wrapping_sub(operand & and_mask) & test_mask) != 0;
    let result = ((std::num::Wrapping(value & and_mask) - std::num::Wrapping(operand & and_mask)).0 & test_mask) != 0.into();

    // log!("UTILS", format!("{0} + {1} ({0:#0width$b} - {1:#0width$b}) would carry on bit {2} ? {3}", value, operand, bit_index, result, width = bit_size(value) + 2));

    return result;
}

pub fn check_half_carry_add(old_value: Value, value: Value) -> bool {
    return check_carry_add(old_value, value, 3);
}

pub fn check_half_carry_sub(old_value: Value, value: Value) -> bool {
    return check_carry_sub(old_value, value, 3)
}

pub fn check_half_carry_wide_add(old_value: WideValue, value: WideValue) -> bool {
    return check_carry_add(old_value, value, 11);
}

pub fn check_half_carry_wide_sub(old_value: WideValue, value: WideValue) -> bool {
    return check_carry_sub(old_value, value, 11)
}