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

#[cfg(feature = "alloc")]
extern crate alloc;

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
///   * **Examples:** [`Wrapping<T>`](core::num::Wrapping) where `T:
///     StableLayout`.
/// * Any other layout that is guaranteed by Rust.
///   * **Examples:** `&T` and `&mut T` where `T: Sized`.
///
/// Specifically there are some things that this type **does not** attempt to
/// guarantee. `StableLayout` types _can_ have:
/// * Uninitialized bytes, such as padding bytes.
/// * Invalid bit patterns, such as `bool` and `char`.
///
/// [type-layout]: https://doc.rust-lang.org/stable/reference/type-layout.html
/// [prim]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#primitive-representations
/// [repr-c]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#the-c-representation
/// [repr-transparent]:
/// https://doc.rust-lang.org/stable/reference/type-layout.html#the-transparent-representation
pub unsafe trait StableLayout {}

unsafe impl StableLayout for u8 {}
unsafe impl StableLayout for u16 {}
unsafe impl StableLayout for u32 {}
unsafe impl StableLayout for u64 {}
unsafe impl StableLayout for usize {}

unsafe impl StableLayout for i8 {}
unsafe impl StableLayout for i16 {}
unsafe impl StableLayout for i32 {}
unsafe impl StableLayout for i64 {}
unsafe impl StableLayout for isize {}

unsafe impl StableLayout for f32 {}
unsafe impl StableLayout for f64 {}

unsafe impl StableLayout for bool {}
unsafe impl StableLayout for char {}
unsafe impl StableLayout for () {}

use core::marker::PhantomData;
/// `PhantomData` is a zero-sized type and so technically it could be defined as
/// always being `StableLayout`. However, since `PhantomData` is semantically
/// supposed to indicate to the world that you want to be treated like you're
/// holding some sort of `T`, then we will also require that the `T` be a
/// `StableLayout` type.
unsafe impl<T> StableLayout for PhantomData<T> where T: StableLayout {}

use core::num::Wrapping;
unsafe impl<T> StableLayout for Wrapping<T> where T: StableLayout {}

use core::mem::ManuallyDrop;
unsafe impl<T> StableLayout for ManuallyDrop<T> where T: StableLayout {}

use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
unsafe impl StableLayout for NonZeroU8 {}
unsafe impl StableLayout for NonZeroU16 {}
unsafe impl StableLayout for NonZeroU32 {}
unsafe impl StableLayout for NonZeroU64 {}
unsafe impl StableLayout for NonZeroUsize {}
unsafe impl StableLayout for Option<NonZeroU8> {}
unsafe impl StableLayout for Option<NonZeroU16> {}
unsafe impl StableLayout for Option<NonZeroU32> {}
unsafe impl StableLayout for Option<NonZeroU64> {}
unsafe impl StableLayout for Option<NonZeroUsize> {}

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
unsafe impl StableLayout for NonZeroI8 {}
unsafe impl StableLayout for NonZeroI16 {}
unsafe impl StableLayout for NonZeroI32 {}
unsafe impl StableLayout for NonZeroI64 {}
unsafe impl StableLayout for NonZeroIsize {}
unsafe impl StableLayout for Option<NonZeroI8> {}
unsafe impl StableLayout for Option<NonZeroI16> {}
unsafe impl StableLayout for Option<NonZeroI32> {}
unsafe impl StableLayout for Option<NonZeroI64> {}
unsafe impl StableLayout for Option<NonZeroIsize> {}

// Note(Lokathor): Technically the pointer itself is stable with just `Sized`,
// even with if the pointed to data isn't stable. However, it's essentially
// impossible to utilize the power of StableLayout if the pointed to data isn't
// stable, so we just require that. If you want to avoid our extra rule here, go
// make your own crate.

// Note(danielhenrymantilla): Technically the `Sized` isn't required here, since
// `Sized` is "on by default", and we'd use `?Sized` if we wanted to turn it
// off, but it's nice to be extra explicit about our expectations.

unsafe impl<T> StableLayout for &T where T: Sized + StableLayout {}
unsafe impl<T> StableLayout for Option<&T> where T: Sized + StableLayout {}

unsafe impl<T> StableLayout for &mut T where T: Sized + StableLayout {}
unsafe impl<T> StableLayout for Option<&mut T> where T: Sized + StableLayout {}

unsafe impl<T> StableLayout for *const T where T: Sized + StableLayout {}
unsafe impl<T> StableLayout for *mut T where T: Sized + StableLayout {}

use core::ptr::NonNull;
unsafe impl<T> StableLayout for NonNull<T> where T: Sized + StableLayout {}
unsafe impl<T> StableLayout for Option<NonNull<T>> where T: Sized + StableLayout {}

use core::cell::{Cell, UnsafeCell};
unsafe impl<T> StableLayout for UnsafeCell<T> where T: StableLayout {}
unsafe impl<T> StableLayout for Cell<T> where T: StableLayout {}

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
unsafe impl<T> StableLayout for Box<T> where T: Sized + StableLayout {}
#[cfg(feature = "alloc")]
unsafe impl<T> StableLayout for Option<Box<T>> where T: Sized + StableLayout {}

macro_rules! impl_unsafe_marker_for_array {
  ( $marker:ident , $( $n:expr ),* ) => {
    $(unsafe impl<T> $marker for [T; $n] where T: $marker {})*
  }
}
#[rustfmt::skip]
impl_unsafe_marker_for_array!(
  StableLayout, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
  48, 64, 96, 128, 256, 512, 1024, 2048, 4096
);

#[cfg(target_arch = "x86")]
use core::arch::x86;
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m128i {}
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m128 {}
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m128d {}
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m256i {}
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m256 {}
#[cfg(target_arch = "x86")]
unsafe impl StableLayout for x86::__m256d {}

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64;
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for x86_64::__m256d {}
