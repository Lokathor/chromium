#![cfg(feature = "unsafe_alloc")]

use super::StableLayout;
use alloc::vec::Vec;
use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
  slice,
};

// General Safety Note: The soundness of the `StableVec` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for the parts of a [`Vec`](alloc::vec::Vec) with a stable layout.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*mut T`, `usize`, `usize`.
/// * **Soundness Invariants**
///   * The `*mut T` must point to the start of a valid `Vec<T>` allocation.
///   * The first `usize` must be the correct length of that valid `Vec<T>`.
///   * The second `usize` must be the correct capacity of that valid `Vec<T>`.
///   * The memory is owned by the `StableVec` and is allocated from Rust's
///     Global Allocator.
///     * You must not turn this type back into a `Vec` in a Rust runtime with a
///       different global allocator than the one is was created with. At the
///       moment (2020-03-06) it happens to be the case that the default Rust
///       global allocator is process-wide on Windows / Mac / Linux.
///
/// If you drop a `StableVec` without turning it back into a `Vec` then the
/// memory leaks.
///
/// When you use this type with the C ABI, remember that the C ABI **does not**
/// support generic types or `repr(Rust)` types!
///
/// If you select a particular type for `T` that is compatible with the C ABI,
/// such as `u8` or `i32`, then that particular monomorphization of
/// `SharedSlice` will be C ABI compatible as well. For example, if your element
/// type were `u8` then it would be equivalent layout to the following C
/// declaration:
///
/// ```c
/// #include <stdint.h>
/// // Identical layout to `StableVec<u8>`
/// typedef struct {
///   uint8_t *ptr;
///   uintptr_t len;
///   uintptr_t cap;
/// } StableVec_u8;
/// ```
#[repr(C)]
pub struct StableVec<T>
where
  T: StableLayout,
{
  ptr: *mut T,
  len: usize,
  cap: usize,
}

unsafe impl<T: StableLayout> StableLayout for StableVec<T> {}

impl<T> Deref for StableVec<T>
where
  T: StableLayout,
{
  type Target = [T];
  #[inline(always)]
  fn deref(&self) -> &[T] {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts(self.ptr, self.len) }
  }
}

impl<T> DerefMut for StableVec<T>
where
  T: StableLayout,
{
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut [T] {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
  }
}

impl<T: Debug> Debug for StableVec<T>
where
  T: StableLayout,
{
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<T> From<Vec<T>> for StableVec<T>
where
  T: StableLayout,
{
  fn from(vec: Vec<T>) -> Self {
    let mut md_vec = core::mem::ManuallyDrop::new(vec);
    let cap = md_vec.capacity();
    let len = md_vec.len();
    let ptr = md_vec.as_mut_ptr();
    Self { ptr, len, cap }
  }
}

impl<T> From<StableVec<T>> for Vec<T>
where
  T: StableLayout,
{
  fn from(sv: StableVec<T>) -> Self {
    // Safety: See note at the top of the module.
    unsafe { Vec::from_raw_parts(sv.ptr, sv.len, sv.cap) }
  }
}

impl<T> Default for StableVec<T>
where
  T: StableLayout,
{
  /// Defaults to an empty vec.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let sv: StableVec<i32> = StableVec::default();
  /// assert_eq!(sv.len(), 0);
  /// ```
  #[inline(always)]
  fn default() -> Self {
    Self::from(Vec::default())
  }
}
