use core::{
  fmt::Debug,
  marker::PhantomData,
  ops::{Deref, DerefMut},
  slice, str,
};

use super::StableLayout;

// General Safety Note: The soundness of the `UniqueStr` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for **unique** `str` views with a stable layout.
///
/// This is a `repr(C)` variant of `&mut str`. If you want a variant of `&str`
/// then you should use [`UniqueStr`](crate::UniqueStr) instead.
///
/// Rationale for using this type is given in the crate level docs.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*mut u8` and then a `usize`.
/// * **Soundness Invariants**
///   * The `*mut u8` must point to the start of a valid `&mut str`.
///   * The `usize` must be the correct length of that valid `&mut str`.
///   * For as long as the `UniqueStr` exists the memory in question has a
///     unique borrow over it (tracked via `PhantomData`).
///   * The memory must contain value UTF-8 data.
///
/// This type matches up with the following C layout:
/// ```c
/// #include <stdint.h>
/// // Identical layout to `UniqueStr<'a>`
/// typedef struct {
///   uint8_t *ptr;
///   uintptr_t len;
/// } UniqueStr;
/// ```
#[repr(C)]
pub struct UniqueStr<'a> {
  ptr: *mut u8,
  len: usize,
  life: PhantomData<&'a mut str>,
}

unsafe impl<'a> StableLayout for UniqueStr<'a> {}

impl<'a> Debug for UniqueStr<'a> {
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<'a> Clone for UniqueStr<'a> {
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a> Copy for UniqueStr<'a> {}

impl<'a> Default for UniqueStr<'a> {
  /// Defaults to an empty string.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let unique: UniqueStr<'static> = UniqueStr::default();
  /// assert_eq!(unique.len(), "".len());
  /// ```
  #[inline(always)]
  fn default() -> Self {
    let life = PhantomData;
    let len = 0;
    let ptr = core::ptr::NonNull::dangling().as_ptr();
    Self { ptr, len, life }
  }
}

impl<'a> Deref for UniqueStr<'a> {
  type Target = str;
  #[inline(always)]
  fn deref(&self) -> &str {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len))
    }
  }
}

impl<'a> DerefMut for UniqueStr<'a> {
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

impl<'a> From<&'a mut str> for UniqueStr<'a> {
  #[inline(always)]
  fn from(s: &'a mut str) -> Self {
    let life = PhantomData;
    let len = s.len();
    let ptr = s.as_mut_ptr();
    Self { ptr, len, life }
  }
}

impl<'a> From<UniqueStr<'a>> for &'a mut str {
  #[inline(always)]
  fn from(unique: UniqueStr<'a>) -> Self {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(
        unique.ptr, unique.len,
      ))
    }
  }
}
