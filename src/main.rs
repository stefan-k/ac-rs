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
#![feature(deadline_api)]

// #[macro_use]
// extern crate failure;
extern crate libc;
extern crate rand;

mod allocator;
// #[macro_use]
// mod errors;
mod executor;

use self::allocator::LimitedAllocator;
use self::executor::run_with_limits;
use std::alloc::System;
use std::time::Duration;

#[global_allocator]
pub static GLOBAL: LimitedAllocator<System> = LimitedAllocator::new(System);

#[inline(never)]
fn dummy(a: f64, b: f64, c: f64) -> u64 {
    // use rand::{thread_rng, Rng};
    // let mut rng = thread_rng();
    // let n: u64 = rng.gen_range(1, 100);
    // let v: Vec<f64> = Vec::with_capacity(50);
    let n = 0;
    let time = (a * b - c - (b - a) * a).abs() as u64 + n;
    std::thread::sleep(Duration::from_millis(time));
    time
}

fn main() {
    let f = || {
        let a = 10.0;
        let b = 2.0;
        let c = 4.0;
        let t = dummy(a, b, c);
        t
    };

    let stats = run_with_limits(f, Duration::from_millis(96), 1000);
    println!("{:?}", stats);

    // println!(
    //     "Time: {:?} ms; Maximum mem usage: {} / {} bytes; Remaining mem: {} bytes",
    //     stats.time,
    //     stats.max_memory,
    //     GLOBAL.limit(),
    //     GLOBAL.get(),
    // );

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
