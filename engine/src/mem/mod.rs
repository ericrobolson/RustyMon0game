pub use alloc::boxed::Box;
use alloc::vec::*;

pub struct StaticBuffer<T> {
    items: Vec<T>,
    capactity: usize,
}

impl<T> StaticBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&mut self, item: T) -> Result<(), BufferError> {
        todo!()
    }

    pub fn pop_head(&mut self) -> Option<T> {
        todo!()
    }
}

pub enum BufferError {
    BufferFull,
}
