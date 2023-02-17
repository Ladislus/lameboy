macro_rules! template_inc_wide {
    ($field: expr) => {
        unsafe { $field += 1; }
    };
}

macro_rules! template_dec_wide {
    ($field: expr) => {
        unsafe { $field -= 1; }
    };
}

macro_rules! template_inc_value {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value: Value = $field;
            let new_value: Value = old_value + 1;
            $field = new_value;

            // Operation flags
            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 3.
            $memory.registers.set_half_carry_flag(check_half_carry_add(old_value, 1));
        }
    };
}

macro_rules! template_dec_value {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $field;
            let new_value = old_value - 1;
            $field = new_value;

            // Operation flags
            $memory.registers.set_zero_flag(new_value == 0);
            $memory.registers.set_subtraction_flag(true);
            // H => Set if borrow from bit 4.
            $memory.registers.set_half_carry_flag(check_half_carry_sub(old_value, 1));
        }
    };
}

macro_rules! template_ld {
    ($field: expr, $value: ident) => {
        $field = $value;
    };
    ($field: expr, $value: expr) => {
        unsafe { $field = $value; }
    };
}

macro_rules! template_add_a {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_a();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_add(value);

            $memory.registers.set_a(result);

            $memory.registers.set_zero_flag(result == 0);
            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 3.
            $memory.registers.set_half_carry_flag(check_half_carry_add(old_value, value));
            // C => Set if overflow from bit 15.
            $memory.registers.set_carry_flag(has_overflown);
        }
    };
}

macro_rules! template_add_hl {
    ($memory: expr, $field: expr) => {
        unsafe {
            let old_value = $memory.registers.get_hl();
            let value = $field;

            let (result, has_overflown) = old_value.overflowing_add(value);

            $memory.registers.set_hl(result);

            $memory.registers.set_subtraction_flag(false);
            // H => Set if overflow from bit 11.
            $memory.registers.set_half_carry_flag(check_half_carry_wide_add(old_value, value));
            // C => Set if overflow from bit 15.
            $memory.registers.set_carry_flag(has_overflown);
        }
    };
}

// Trick to export macro to current crate, without using "macro_export"
#[allow(unused_imports)]
pub(super) use {template_inc_wide, template_dec_wide, template_inc_value, template_dec_value, template_ld, template_add_hl, template_add_a};