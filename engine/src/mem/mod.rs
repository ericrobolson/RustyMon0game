pub use std::boxed::Box;
use std::vec::*;

pub struct StaticBuffer<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> StaticBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), BufferError> {
        if self.items.len() >= self.capacity {
            Err(BufferError::BufferFull)
        } else {
            Ok(())
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}

pub enum BufferError {
    BufferFull,
}
