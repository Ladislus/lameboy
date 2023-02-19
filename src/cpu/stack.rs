use crate::utils::conversions::{pair_to_wide, wide_to_pair};
use crate::utils::log::log;
use crate::utils::types::{FarAddress, Value, WideValue};

type StackType = Vec<Value>;

pub struct Stack {
    stack: StackType,
    base_address: FarAddress,
    size: usize,
}

impl Stack {
    pub fn new(base_addr: FarAddress) -> Stack {
        return Stack {
            stack: StackType::new(),
            base_address: base_addr,
            size: 0,
        }
    }

    fn addr_to_index(&self, addr: FarAddress) -> usize {
        let (result, has_overflown) = self.base_address.overflowing_sub(addr as FarAddress);

        debug_assert!(!has_overflown);
        log!("STACK", format!("Address: {:#X} correspond to index {}", addr, result));

        return result as usize;
    }

    fn push(&mut self, sp: &mut FarAddress, value: Value) {

        *sp -= 1;

        let index = self.addr_to_index(*sp);
        let max_index = self.size - 1;

        // Assert that, at most, the is 1 element ahead of the stack (the top)
        debug_assert!(index <= (max_index + 1));

        // If the space is already full, append to the stack (i.e add 1 more space)
        if index > max_index {
            self.stack.push(value);
        // Else use already existing space
        } else {
            self.stack[index] = value;
        }

        self.size += 1;
    }

    pub fn push_wide(&mut self, sp: &mut FarAddress, value: WideValue) {
        let (high, low) = wide_to_pair(value);
        self.push(sp, high);
        self.push(sp, low);
    }

    pub fn pop(&mut self, sp: &mut FarAddress) -> Value {
        let index = self.addr_to_index(*sp);
        let max_index = self.size - 1;

        // Assert index is in range
        debug_assert!(index <= max_index);

        let value = self.stack[index];

        *sp += 1;

        return value;
    }

    pub fn pop_wide(&mut self, sp: &mut FarAddress) -> WideValue {
        let low = self.pop(sp);
        let high = self.pop(sp);

        return pair_to_wide(high, low);
    }
}