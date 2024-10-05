use std::collections::HashMap;

pub struct StackAllocator {
    // Map between a pseudo register name ans a stack offset.
    mapping: HashMap<String, i64>,

    max_offset: i64,
}

impl StackAllocator {
    pub fn new() -> StackAllocator {
        StackAllocator {
            max_offset: 0,
            mapping: HashMap::new(),
        }
    }

    pub fn get_stack_offset(&mut self, pseudo_register: &str) -> i64 {
        if let Some(offset) = self.mapping.get(pseudo_register) {
            *offset
        } else {
            self.max_offset -= 4;
            self.mapping.insert(pseudo_register.to_owned(), self.max_offset);
            self.max_offset
        }
    }

    pub fn stack_size(&self) -> i64 {
        -self.max_offset
    }
}