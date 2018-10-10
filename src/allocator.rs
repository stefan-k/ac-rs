// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Allocator wrapper
//!
//! Based on this reddit post by user 0b_0101_001_1010:
//! https://www.reddit.com/r/rust/comments/8z83wc/is_there_any_way_to_benchmark_memory_usage_in_rust/e2h4dp9

use std::alloc::{GlobalAlloc, Layout};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct LimitedAllocator<T: GlobalAlloc> {
    allocator: T,
    mem: AtomicU64,
    limit: AtomicU64,
    maximum: AtomicU64,
    aborting: AtomicBool,
}

fn abort_hook(_l: Layout) {
    std::io::stderr()
        .write(b"Memory limit exceeded!\n")
        .unwrap();
    panic!("oh no")
}

unsafe impl<T: GlobalAlloc> GlobalAlloc for LimitedAllocator<T> {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let ls = l.size() as u64;
        let old = self.mem.fetch_add(l.size() as u64, Ordering::SeqCst);
        let sum = old + ls;

        self.maximum.fetch_max(sum, Ordering::SeqCst);

        if !self.aborting.load(Ordering::SeqCst) {
            if sum > self.limit.load(Ordering::SeqCst) {
                self.aborting.store(true, Ordering::SeqCst);
                let np: *const u8 = std::ptr::null();
                return np as *mut u8;
            }
        }

        self.allocator.alloc(l)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        self.allocator.dealloc(ptr, l);
        self.mem.fetch_sub(l.size() as u64, Ordering::SeqCst);
    }
}

impl<T: GlobalAlloc> LimitedAllocator<T> {
    pub const fn new(a: T) -> Self {
        LimitedAllocator {
            allocator: a,
            mem: AtomicU64::new(0),
            limit: AtomicU64::new(std::u64::MAX),
            maximum: AtomicU64::new(0),
            aborting: AtomicBool::new(false),
        }
    }

    pub fn reset(&self) {
        std::alloc::set_alloc_error_hook(abort_hook);
        self.mem.store(0, Ordering::SeqCst);
        self.maximum.store(0, Ordering::SeqCst);
        self.aborting.store(false, Ordering::SeqCst);
    }

    pub fn get(&self) -> u64 {
        self.mem.load(Ordering::SeqCst)
    }

    pub fn set_limit(&self, limit: u64) {
        self.limit.store(limit, Ordering::SeqCst);
    }

    pub fn limit(&self) -> u64 {
        self.limit.load(Ordering::SeqCst)
    }

    pub fn max(&self) -> u64 {
        self.maximum.load(Ordering::SeqCst)
    }
}
