use core::{fmt::Debug, marker::PhantomData, ops::Deref};
use core::ops::DerefMut;
// A rare occurrence of Lokathor importing a module!
use core::slice;

// General Safety Note: The soundness of the `CUniqueSlice` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for **unique** slices with a stable layout.
///
/// This is a `repr(C)` variant of `&mut [T]`. If you want a variant of `&[T]`
/// then you should use [`CSharedSlice`](crate::CSharedSlice) instead.
///
/// Rationale for using this type is given in the crate level docs.
/// 
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a `*mut T` and then a `usize`.
/// * **Soundness Invariants**
///   * The `*mut T` must point to the start of a valid `&mut [T]`.
///   * The `usize` must be the correct length of that valid `&mut [T]`.
///   * For as long as the `CUniqueSlice` exists the memory in question has a
///     unique borrow over it (tracked via `PhantomData`).
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
/// // Identical layout to `CUniqueSlice<'a, u8>`
/// typedef struct {
///   uint8_t *ptr;
///   uintptr_t len;
/// } CUniqueSlice_u8;
/// ```
#[repr(C)]
pub struct CUniqueSlice<'a, T> {
  ptr: *mut T,
  len: usize,
  life: PhantomData<MutSlice<'a,T>>,
}

#[repr(transparent)]
struct MutSlice<'a,T>(&'a mut [T]);

impl<'a, T: Debug> Debug for CUniqueSlice<'a, T> {
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<'a, T> Default for CUniqueSlice<'a, T> {
  /// Defaults to an empty slice.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CUniqueSlice<'static, i32> = CUniqueSlice::default();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  fn default() -> Self {
    Self::empty_slice()
  }
}

impl<'a, T> Deref for CUniqueSlice<'a, T> {
  type Target = [T];
  #[inline(always)]
  fn deref(&self) -> &[T] {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts(self.ptr, self.len) }
  }
}

impl<'a, T> DerefMut for CUniqueSlice<'a, T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut [T] {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
  }
}

impl<'a, T> From<&'a mut [T]> for CUniqueSlice<'a, T> {
  fn from(sli: &'a mut [T]) -> Self {
    let life = PhantomData;
    let len = sli.len();
    let ptr = sli.as_mut_ptr();
    Self { ptr, len, life }
  }
}

impl<'a, T> From<CUniqueSlice<'a, T>> for &'a mut [T] {
  fn from(c_shared: CUniqueSlice<'a, T>) -> Self {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts_mut(c_shared.ptr, c_shared.len) }
  }
}

impl<'a, T> CUniqueSlice<'a, T> {
  /// Gives an empty slice.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CUniqueSlice<'static, i32> = CUniqueSlice::empty_slice();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  pub const fn empty_slice() -> Self {
    let life = PhantomData;
    let len = 0;
    let ptr = core::ptr::NonNull::dangling().as_ptr();
    Self { ptr, len, life }
  }
}

