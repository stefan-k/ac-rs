// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub struct FakeAllocator {}

impl FakeAllocator {
    pub fn reset(&self) {}

    pub fn get(&self) -> u64 {
        0
    }

    pub fn set_limit(&self, _limit: u64) {}

    pub fn limit(&self) -> u64 {
        0
    }

    pub fn max(&self) -> u64 {
        0
    }
}
