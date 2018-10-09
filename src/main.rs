// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! AC
//!

// #![warn(missing_docs)]
#![feature(integer_atomics, const_fn)]
#![feature(alloc_error_hook)]
#![feature(allocator_api)]

mod allocator;
use self::allocator::LimitedAllocator;
use std::alloc::System;

#[global_allocator]
static GLOBAL: LimitedAllocator<System> = LimitedAllocator::new(System);

fn dummy(a: f64, b: f64, c: f64) -> u64 {
    let time = (a * b - c - (b - a) * a).abs() as u64;
    std::thread::sleep(std::time::Duration::from_millis(time));
    time
}

fn main() {
    GLOBAL.reset();
    GLOBAL.set_limit(1150);
    let result = std::panic::catch_unwind(|| {
        // panic!("fuck");
        let a = 10.0;
        let b = 2.0;
        let c = 4.0;
        dummy(a, b, c)
    });

    if result.is_err() {
        println!("fuck2");
    }

    if result.is_ok() {
        println!(
            "Time: {:?} ms; Mem usage: {} bytes",
            result.unwrap(),
            GLOBAL.get()
        );
    }
}
