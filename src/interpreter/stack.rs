use crate::types::{Address, Bytes32};
use ethereum_types::U256;

/// EVM interpreter stack limit.
pub const STACK_LIMIT: usize = 1024;

/// EVM stack with `STACK_LIMIT` capacity of words.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    /// The underlying 32-byte words of the stack.
    items: Vec<Bytes32>,
    /// The max depth of the stack.
    max_depth: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(STACK_LIMIT),
            max_depth: STACK_LIMIT,
        }
    }

    // Stack Operations

    pub fn push(&mut self, value: Bytes32) {
        if self.items.len() == self.max_depth {
            panic!("Stack overflow");
        }
        println!(" > PUSH {:#X}", value);
        self.items.push(value);
    }

    pub fn push_u256(&mut self, number: U256) {
        self.push(Bytes32::from_u256(number));
    }

    pub fn push_address(&mut self, address: Address) {
        self.push(Bytes32::from_address(address));
    }

    pub fn push_usize(&mut self, size: usize) {
        self.push_u256(size.into());
    }

    pub fn pop(&mut self) -> Bytes32 {
        if self.items.is_empty() {
            panic!("Stack underflow");
        }

        self.items.pop().unwrap()
    }

    pub fn swap(&mut self, depth: usize) {
        let stack_depth = self.depth();
        if depth >= stack_depth {
            panic!("Stack underflow");
        }

        let index = stack_depth - depth - 1;
        self.items.swap(index, stack_depth - 1);
    }

    // Stack Getters

    pub fn items(&self) -> &Vec<Bytes32> {
        &self.items
    }

    pub fn get_item(&self, depth: usize) -> Option<Bytes32> {
        if depth >= self.items.len() {
            None
        } else {
            Some(self.items[depth].clone())
        }
    }

    pub fn deref_items(&self) -> Vec<Bytes32> {
        let mut items = self.items.clone();
        items.reverse();
        items
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn depth(&self) -> usize {
        self.items.len()
    }
}