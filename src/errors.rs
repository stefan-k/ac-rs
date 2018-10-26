// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Errors

#[derive(Debug, Fail)]
pub enum AcError {
    // /// Indicates that the target algorithm panicked
    // #[fail(display = "TargetAlgorithmPanic: {}", text)]
    // TargetAlgorithmPanic { text: String },

    // /// Indicates that the target algorithm crashed (means: Not there anymore, but no clue why)
    // #[fail(display = "TargetAlgorithmCrash: {}", text)]
    // TargetAlgorithmCrash { text: String },
    /// Something else
    #[fail(display = "Whatever!")]
    Whatever {},
}

#[macro_export]
macro_rules! make_err {
    () => {
        Err(AcError::Whatever { }.into())
    };
    ($type:ty, $text:expr) => {
        Err(AcError::$type { $text.to_string() }.into())
    };
}
