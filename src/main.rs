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
#![feature(atomic_min_max)]
#![feature(alloc_error_hook)]
#![feature(allocator_api)]

extern crate rand;

mod allocator;
use self::allocator::LimitedAllocator;
use rand::{thread_rng, Rng};
use std::alloc::System;

#[global_allocator]
static GLOBAL: LimitedAllocator<System> = LimitedAllocator::new(System);

#[inline(never)]
fn dummy(a: f64, b: f64, c: f64) -> u64 {
    let mut rng = thread_rng();
    let n: u64 = rng.gen_range(1, 100);
    let time = (a * b - c - (b - a) * a).abs() as u64 + n;
    std::thread::sleep(std::time::Duration::from_millis(time));
    time
}

fn main() {
    GLOBAL.reset();
    GLOBAL.set_limit(10000);

    {
        let a = 10.0;
        let b = 2.0;
        let c = 4.0;
        let t = dummy(a, b, c);
        println!(
            "Time: {:?} ms; Maximum mem usage: {} / {} bytes; Remaining mem: {} bytes",
            t,
            GLOBAL.max(),
            GLOBAL.limit(),
            GLOBAL.get(),
        );
    }
    println!(
        "AFTER BLOCK: Maximum mem usage: {} / {} bytes; Remaining mem: {} bytes",
        GLOBAL.max(),
        GLOBAL.limit(),
        GLOBAL.get(),
    );

    // let result = std::panic::catch_unwind(|| {
    //     // panic!("flup");
    //     let a = 10.0;
    //     let b = 2.0;
    //     let c = 4.0;
    //     dummy(a, b, c)
    // });
    //
    // if result.is_err() {
    //     println!("flup2");
    // }
    //
    // if result.is_ok() {
    //     println!(
    //         "Time: {:?} ms; Maximum mem usage: {} bytes",
    //         result.unwrap(),
    //         GLOBAL.max()
    //     );
    // }
}
