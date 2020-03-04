use core::{fmt::Debug, marker::PhantomData, ops::Deref};
// A rare occurrence of Lokathor importing a module!
use core::slice;

use super::StableLayout;

// General Safety Note: The soundness of the `CSharedSlice` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for **shared** slices with a stable layout.
///
/// This is a `repr(C)` variant of `&[T]`. If you want a variant of `&mut [T]`
/// then you should use [`CUniqueSlice`](crate::CUniqueSlice) instead.
///
/// Rationale for using this type is given in the crate level docs.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*const T` and then a `usize`.
/// * **Soundness Invariants**
///   * The `*const T` must point to the start of a valid `&[T]`.
///   * The `usize` must be the correct length of that valid `&[T]`.
///   * For as long as the `CSharedSlice` exists the memory in question has a
///     shared borrow over it (tracked via `PhantomData`).
///
/// When you use this type with the C ABI, remember that the C ABI **does not**
/// support generic types or `repr(Rust)` types!
///
/// If you select a particular type for `T` that is compatible with the C ABI,
/// such as `u8` or `i32`, then that particular monomorphization of
/// `CSharedSlice` will be C ABI compatible as well. For example, if your
/// element type were `u8` then it would be equivalent layout to the following C
/// declaration:
/// ```c
/// #include <stdint.h>
/// // Identical layout to `CSharedSlice<'a, u8>`
/// typedef struct {
///   uint8_t const *ptr;
///   uintptr_t len;
/// } CSharedSlice_u8;
/// ```
#[repr(C)]
pub struct CSharedSlice<'a, T>
where
  T: StableLayout,
{
  ptr: *const T,
  len: usize,
  life: PhantomData<&'a [T]>,
}

impl<'a, T: Debug> Debug for CSharedSlice<'a, T>
where
  T: StableLayout,
{
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<'a, T> Clone for CSharedSlice<'a, T>
where
  T: StableLayout,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    // Note(Lokathor): We can't derive Clone and Copy or CSharedSlice will only
    // be Clone and Copy when T is clone and Copy. However, CSharedSlice is
    // actually a *slice of* T, so it is always Clone and Copy even if T is not.
    *self
  }
}

impl<'a, T> Copy for CSharedSlice<'a, T> where T: StableLayout {}

impl<'a, T> Default for CSharedSlice<'a, T>
where
  T: StableLayout,
{
  /// Defaults to an empty slice.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CSharedSlice<'static, i32> = CSharedSlice::default();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  fn default() -> Self {
    let life = PhantomData;
    let len = 0;
    let ptr = core::ptr::NonNull::dangling().as_ptr();
    Self { ptr, len, life }
  }
}

impl<'a, T> Deref for CSharedSlice<'a, T>
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

impl<'a, T> From<&'a [T]> for CSharedSlice<'a, T>
where
  T: StableLayout,
{
  fn from(sli: &'a [T]) -> Self {
    let life = PhantomData;
    let len = sli.len();
    let ptr = sli.as_ptr();
    Self { ptr, len, life }
  }
}

impl<'a, T> From<CSharedSlice<'a, T>> for &'a [T]
where
  T: StableLayout,
{
  fn from(c_shared: CSharedSlice<'a, T>) -> Self {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts(c_shared.ptr, c_shared.len) }
  }
}

/*
impl<'a, T> CSharedSlice<'a, T>  where T: StableLayout{
  /// Gives an empty slice as a `const` value.
  ///
  /// Some day this will be `const`
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CSharedSlice<'static, i32> = CSharedSlice::empty_slice();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  pub fn empty_slice() -> Self {
    let life = PhantomData;
    let len = 0;
    let ptr = core::ptr::NonNull::dangling().as_ptr();
    Self { ptr, len, life }
  }
}
*/
