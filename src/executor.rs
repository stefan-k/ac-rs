// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Execute code with runtime and memory limits

// use crate::errors::*;
// use failure::Error;
use crate::GLOBAL;
use libc::pthread_cancel;
use std::os::unix::thread::JoinHandleExt;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum AcStatus {
    Success,
    // MemoryLimitExceeded,
    TimeLimitExceeded,
    TargetAlgorithmPanic,
    TargetAlgorithmSpawnFail,
    TargetAlgorithmCommunicationFail,
}

#[derive(Debug)]
pub struct AcStats<T> {
    pub status: AcStatus,
    pub max_memory: u64,
    pub time: Duration,
    pub output: Option<T>,
}

impl<T> AcStats<T> {
    pub fn new(status: AcStatus, max_memory: u64, time: Duration, output: Option<T>) -> Self {
        AcStats {
            status,
            max_memory,
            time,
            output,
        }
    }
}

pub fn run_with_limits<F, T>(f: F, time_limit: Duration, memory_limit: u64) -> AcStats<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let mut status = AcStatus::Success;
    let max_memory: u64;
    let time: Duration;
    let mut output: Option<T> = None;

    GLOBAL.reset();
    GLOBAL.set_limit(memory_limit);

    let (tx, rx) = channel();
    let builder = thread::Builder::new();

    let handler = builder.spawn(move || {
        let now = Instant::now();
        let result = f();
        let elapsed = now.elapsed();
        tx.send(true).unwrap();
        (elapsed, result)
    });

    match handler {
        Ok(handle) => {
            let join_handle: thread::JoinHandle<_> = handle;
            match rx.recv_deadline(Instant::now() + time_limit) {
                Ok(_) => {
                    match join_handle.join() {
                        Ok((elapsed_time, res)) => {
                            // Everything is fine!
                            max_memory = GLOBAL.max();
                            time = elapsed_time;
                            output = Some(res);
                        }
                        Err(_) => {
                            // There was a panic in the target algorithm
                            max_memory = GLOBAL.max();
                            time = Duration::from_millis(0);
                            status = AcStatus::TargetAlgorithmPanic;
                        }
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    // Ran out of time
                    max_memory = GLOBAL.max();
                    time = time_limit;
                    status = AcStatus::TimeLimitExceeded;

                    // Kill thread
                    let thread = join_handle.into_pthread_t();
                    unsafe {
                        pthread_cancel(thread);
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    // Channel disconnected, something bad happend!
                    max_memory = GLOBAL.max();
                    time = Duration::from_millis(0);
                    status = AcStatus::TargetAlgorithmCommunicationFail;

                    // Kill thread
                    let thread = join_handle.into_pthread_t();
                    unsafe {
                        pthread_cancel(thread);
                    }
                }
            }
        }
        Err(_) => {
            max_memory = GLOBAL.max();
            time = Duration::from_millis(0);
            status = AcStatus::TargetAlgorithmSpawnFail;
        }
    }

    AcStats::new(status, max_memory, time, output)
}
