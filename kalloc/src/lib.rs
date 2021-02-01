#![no_std]

//! A custom "Bump" allocator for tracking memory usage within Kalavar

use core::alloc::{Layout, GlobalAlloc};
use core::ptr::*;

pub struct Kalloc {
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl Kalloc {
    /// Create a new memory allocator
    pub const fn new() -> Self {
        Kalloc {
            start: 0,
            end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.start = heap_start;
        self.end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Kalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // let alloc_start = self.next;
        // self.next = alloc_start + layout.size();
        // self.allocations += 1;
        // alloc_start as *mut u8
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}