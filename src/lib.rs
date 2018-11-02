// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! AC
//!

// #![warn(missing_docs)]
#![feature(integer_atomics)]
#![feature(atomic_min_max)]
#![feature(alloc_error_hook)]
#![feature(allocator_api)]
#![feature(deadline_api)]
#![feature(const_fn)]

extern crate libc;
extern crate state;

#[cfg(feature = "on")]
mod allocator;
mod executor;
#[cfg(not(feature = "on"))]
mod fake_allocator;

pub use self::executor::run_with_limits;
#[cfg(feature = "on")]
use crate::allocator::LimitedAllocator;
#[cfg(not(feature = "on"))]
use crate::fake_allocator::FakeAllocator;
#[cfg(feature = "on")]
use std::alloc::System;

#[cfg(feature = "on")]
#[global_allocator]
pub static GLOBAL: LimitedAllocator<System> = LimitedAllocator::new(System);

#[cfg(not(feature = "on"))]
pub static GLOBAL: FakeAllocator = FakeAllocator {};

#[cfg(feature = "on")]
#[macro_export]
macro_rules! ac {
    ($var:ident =>  $default:expr) => {
        CONFIG.get().$var
    };
}

#[cfg(not(feature = "on"))]
#[macro_export]
macro_rules! ac {
    ($var:ident =>  $default:expr) => {
        $default
    };
}
