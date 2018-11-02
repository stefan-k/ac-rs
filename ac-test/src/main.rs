// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! AC
//!

// #![warn(missing_docs)]

extern crate ac;
extern crate state;

use ac::{ac, run_with_limits};
use state::LocalStorage;
use std::time::Duration;

static CONFIG: LocalStorage<Configuration> = LocalStorage::new();

struct Configuration {
    #[allow(dead_code)]
    x1: f64,
    #[allow(dead_code)]
    x2: f64,
    #[allow(dead_code)]
    x3: f64,
}

#[inline(never)]
fn dummy(a: f64) -> u64 {
    let c = ac!(x2 => 4.0);
    let b = ac!(x3 => 2.0);
    let time = (a * b - c - (b - a) * a).abs() as u64;
    std::thread::sleep(Duration::from_millis(time));
    time
}

fn main() {
    CONFIG.set(|| Configuration {
        x1: 10.0,
        x2: 4.0,
        x3: 2.0,
    });

    let f = || {
        let a = ac!(x1 => 1.0);
        let t = dummy(a);
        t
    };

    let stats = run_with_limits(f, Duration::from_millis(97), 10000);
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
