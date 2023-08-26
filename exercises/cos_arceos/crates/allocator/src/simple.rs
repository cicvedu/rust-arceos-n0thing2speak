//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;
use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    size: usize,
    next: usize,
    allocations: u16,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.size = _size;
        self.next = _start;
        self.allocations = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let lsize = _layout.size();
        if lsize > (self.size - (self.next - self.start)) {
            return Err(crate::AllocError::NoMemory);
        }
        let new_start = self.next;
        self.next += lsize;
        self.allocations += 1;
        return Ok(NonZeroUsize::new(new_start).unwrap());
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.next
    }

    fn available_bytes(&self) -> usize {
        self.size - (self.next - self.start)
    }
}

