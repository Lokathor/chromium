#![cfg(feature = "unsafe_alloc")]

use super::StableLayout;
use alloc::string::String;
use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
  slice, str,
};

// General Safety Note: The soundness of the `StableString` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for the parts of a [`String`](alloc::string::String) with a stable
/// layout.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*mut T`, `usize`, `usize`.
/// * **Soundness Invariants**
///   * The `*mut T` must point to the start of a valid `String` allocation.
///   * The first `usize` must be the correct length of that valid `String`.
///   * The second `usize` must be the correct capacity of that valid `String`.
///   * The memory is owned by the `String` and is allocated from Rust's Global
///     Allocator.
///     * You must not turn this type back into a `String` in a Rust runtime
///       with a different global allocator than the one is was created with. At
///       the moment (2020-03-06) it happens to be the case that the default
///       Rust global allocator is process-wide on Windows / Mac / Linux.
///   * The memory must contain valid UTF-8 data.
///
/// If you drop a `StableString` without turning it back into a `StableString`
/// then the memory leaks.
///
/// ```c
/// #include <stdint.h>
/// // Identical layout to `StableString`
/// typedef struct {
///   uint8_t *ptr;
///   uintptr_t len;
///   uintptr_t cap;
/// } StableString;
/// ```
#[repr(C)]
pub struct StableString {
  ptr: *mut u8,
  len: usize,
  cap: usize,
}

unsafe impl StableLayout for StableString {}

impl Deref for StableString {
  type Target = str;
  #[inline(always)]
  fn deref(&self) -> &str {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len))
    }
  }
}

impl DerefMut for StableString {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut str {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(
        self.ptr, self.len,
      ))
    }
  }
}

impl Debug for StableString {
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl From<String> for StableString {
  fn from(s: String) -> Self {
    let mut md_s = core::mem::ManuallyDrop::new(s);
    let cap = md_s.capacity();
    let len = md_s.len();
    let ptr = md_s.as_mut_ptr();
    Self { ptr, len, cap }
  }
}

impl From<StableString> for String {
  fn from(sv: StableString) -> Self {
    // Safety: See note at the top of the module.
    unsafe { String::from_raw_parts(sv.ptr, sv.len, sv.cap) }
  }
}

impl Default for StableString {
  /// Defaults to an empty vec.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let sv: StableString = StableString::default();
  /// assert_eq!(sv.len(), 0);
  /// ```
  #[inline(always)]
  fn default() -> Self {
    Self::from(String::default())
  }
}
