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
}

// Trick to export macro to current crate, without using "macro_export"
pub(super) use {template_inc_wide, template_dec_wide, template_inc_value, template_dec_value, template_ld};