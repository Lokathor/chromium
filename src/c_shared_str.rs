use core::{fmt::Debug, marker::PhantomData, ops::Deref, slice, str};

use super::StableLayout;

// General Safety Note: The soundness of the `CSharedStr` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for **shared** `str` views with a stable layout.
///
/// This is a `repr(C)` variant of `&str`. If you want a variant of `&mut str`
/// then you should use [`CUniqueStr`](crate::CUniqueStr) instead.
///
/// Rationale for using this type is given in the crate level docs.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*const u8` and then a `usize`.
/// * **Soundness Invariants**
///   * The `*const u8` must point to the start of a valid `&str`.
///   * The `usize` must be the correct length of that valid `&str`.
///   * For as long as the `CSharedStr` exists the memory in question has a
///     shared borrow over it (tracked via `PhantomData`).
///   * The memory must contain value UTF-8 data.
///
/// This type matches up with the following C layout:
/// ```c
/// #include <stdint.h>
/// // Identical layout to `CSharedStr<'a>`
/// typedef struct {
///   uint8_t const *ptr;
///   uintptr_t len;
/// } CSharedStr;
/// ```
#[repr(C)]
pub struct CSharedStr<'a> {
  ptr: *const u8,
  len: usize,
  life: PhantomData<&'a str>,
}

unsafe impl<'a> StableLayout for CSharedStr<'a> {}

impl<'a> Debug for CSharedStr<'a> {
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<'a> Clone for CSharedStr<'a> {
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a> Copy for CSharedStr<'a> {}

impl<'a> Default for CSharedStr<'a> {
  /// Defaults to an empty string.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CSharedStr<'static> = CSharedStr::default();
  /// assert_eq!(c_shared.len(), "".len());
  /// ```
  #[inline(always)]
  fn default() -> Self {
    let life = PhantomData;
    let len = 0;
    let ptr = core::ptr::NonNull::dangling().as_ptr();
    Self { ptr, len, life }
  }
}

impl<'a> Deref for CSharedStr<'a> {
  type Target = str;
  #[inline(always)]
  fn deref(&self) -> &str {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len))
    }
  }
}

impl<'a> From<&'a str> for CSharedStr<'a> {
  #[inline(always)]
  fn from(s: &'a str) -> Self {
    let life = PhantomData;
    let len = s.len();
    let ptr = s.as_ptr();
    Self { ptr, len, life }
  }
}

impl<'a> From<CSharedStr<'a>> for &'a str {
  #[inline(always)]
  fn from(c_shared: CSharedStr<'a>) -> Self {
    // Safety: See note at the top of the module.
    unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(
        c_shared.ptr,
        c_shared.len,
      ))
    }
  }
}
