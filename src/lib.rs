#![no_std]
#![warn(missing_docs)]

//! Chromium helps add some stability to your metal.
//!
//! Specifically, this crate lets you turn select `repr(Rust)` types into a
//! `repr(C)` struct that holds all the data necessary to safely reconstruct the
//! original `repr(Rust)` form.
//!
//! This is primarily of use for sending data from Rust code on one side of a C
//! ABI "FFI" call to other Rust code on the far side of that FFI call. Even if
//! the Rust form of the data changes between compiler versions, because the C
//! ABI is stable each side will be able to turn the information back into
//! whatever it locally needs.
//!
//! You could of course also use this to communicate with non-Rust code if you
//! need to.
//!
//! The types here provide fairly _minimal_ functionality beyond just turning
//! themselves back into their `repr(Rust)` forms. A few basics like `Debug` and
//! `Deref` and so on are provided as appropriate, but for any serious usage
//! you're expected to just change the value back into the Rust form and use the
//! "real" form of the data.
//!
//! ## Features
//!
//! * `alloc` enables support for `Vec`, `String`, and `Box`.
//!   * Note that in this case you **must not** allocations between two
//!     different global allocators.
//!   * As of 2020-03-06 it _happens to be the case_ that the default global
//!     allocators for Windows / Mac / Linux are process wide allocators. If you
//!     change the global allocator things can break. If the rust standard
//!     library changes their global allocator things can break.
//!   * This is a _brittle_ feature, not to be used lightly.

#[cfg(feature = "alloc")]
extern crate alloc;

mod stable_layout;
pub use stable_layout::*;

mod shared_slice;
pub use shared_slice::*;

mod unique_slice;
pub use unique_slice::*;

mod shared_str;
pub use shared_str::*;

mod unique_str;
pub use unique_str::*;

#[cfg(feature = "alloc")]
mod stable_vec;
#[cfg(feature = "alloc")]
pub use stable_vec::*;

#[cfg(feature = "alloc")]
mod stable_string;
#[cfg(feature = "alloc")]
pub use stable_string::*;
