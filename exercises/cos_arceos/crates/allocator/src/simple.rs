//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    start: usize,
    size: usize,
    used: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            used: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.size = _size;
        self.used = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        self.start = _start;
        self.size = _size;
        self.used = 0;
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        if (self.size-self.used)< _layout.size(){
            Err(AllocError::NoMemory)
        }else{
            let ptr = self.start;
            self.start+=_layout.size();
            self.used+=_layout.size();
            Ok(NonZeroUsize::new(ptr).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        if self.size <= 1 {
            self.used = 0;
            self.size = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.used
    }

    fn available_bytes(&self) -> usize {
        self.size-self.used
    }
}