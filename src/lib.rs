#![no_std]
#![warn(missing_docs)]

//! Chromium helps add some stability to your metal.
//!
//! Specifically, this crate lets you turn select `repr(Rust)` types into a
//! `repr(C)` struct that holds all the data necessary to reconstruct the
//! original `repr(Rust)` form.
//!
//! This is primarily of use for sending data from Rust code on one side of a C
//! ABI to other Rust code on the far side of that C ABI. Even if both bits of
//! Rust code were compiled by different compiler versions, because the
//! intermediate `repr(C)` form is stable and fixed it can be used as a
//! communication method.
//!
//! You could of course also use this to communicate with non-Rust code if you
//! need to.
//!
//! The types here provide fairly _minimal_ functionality beyond just turning
//! themselves back into their `repr(Rust)` forms. A few basics like `Debug` and
//! `Deref` and so on are provided as appropriate, but for any serious usage
//! you're expected to just change the value back into the Rust form and use the
//! "real" form of the data.

mod c_shared_slice;
pub use c_shared_slice::*;

mod c_unique_slice;
pub use c_unique_slice::*;
