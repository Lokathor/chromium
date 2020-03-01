#![no_std]
#![warn(missing_docs)]

//! Chromium helps add some stability to your metal, prevents corrosion.

mod c_shared_slice;
pub use c_shared_slice::*;

mod c_unique_slice;
pub use c_unique_slice::*;
