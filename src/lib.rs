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
//! **Currently supported:**
//! * Shared slices, `&[T]`, when `T` has a C-compatible layout.
//! * Unique slices, `&mut [T]`, when `T` has a C-compatible layout.

mod c_shared_slice;
pub use c_shared_slice::*;

mod c_unique_slice;
pub use c_unique_slice::*;

/// Indicates a type with a layout that is stable across Rust compiler versions.
///
/// ## Safety
/// The type's [Type Layout][type-layout] must fit one of the following:
/// * [Primitive][prim] layout types of 64-bits or less.
///   * **Examples:** `i8`, `u32`
/// * Any zero-sized type (ZST).
///   * **Examples:** `()`
/// * [`repr(C)`][repr-c] `struct` or `union` types when all fields are also
///   `StableLayout`.
///   * **Examples:** Most `libc` and `winapi` types.
/// * [`repr(transparent)`][repr-transparent] `struct` or `union` types when the
///   non-ZST field is also `StableLayout`.
///   * **Examples:** [`Wrapping<T>`](core::num::Wrapping) where `T: StableLayout`.
/// * Any other layout that is guaranteed by Rust.
///   * **Examples:** `&T` and `&mut T` where `T: Sized`.
///
/// [type-layout]: https://doc.rust-lang.org/stable/reference/type-layout.html
/// [prim]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#primitive-representations
/// [repr-c]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#the-c-representation
/// [repr-transparent]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#the-transparent-representation
pub unsafe trait StableLayout { }

unsafe impl StableLayout for u8 { }
unsafe impl StableLayout for u16 { }
unsafe impl StableLayout for u32 { }
unsafe impl StableLayout for u64 { }
unsafe impl StableLayout for usize { }

unsafe impl StableLayout for i8 { }
unsafe impl StableLayout for i16 { }
unsafe impl StableLayout for i32 { }
unsafe impl StableLayout for i64 { }
unsafe impl StableLayout for isize { }

unsafe impl StableLayout for bool { }
unsafe impl StableLayout for char { }
unsafe impl StableLayout for () { }

unsafe impl<T> StableLayout for core::num::Wrapping<T> where T: StableLayout { }

unsafe impl<T> StableLayout for &T where T: Sized { }
unsafe impl<T> StableLayout for Option<&T> where T: Sized { }

unsafe impl<T> StableLayout for &mut T where T: Sized { }
unsafe impl<T> StableLayout for Option<&mut T> where T: Sized { }

unsafe impl<T> StableLayout for core::ptr::NonNull<T> where T: Sized { }
unsafe impl<T> StableLayout for Option<core::ptr::NonNull<T>> where T: Sized { }
