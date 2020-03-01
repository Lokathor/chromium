use core::{fmt::Debug, marker::PhantomData, ops::Deref, ptr::NonNull};
// A rare occurrence of Lokathor importing a module!
use core::slice;

// General Safety Note: The soundness of the `CSharedSlice` type is centered
// around the fact that the fields are all private, and so *safe rust* must
// construct values of the type from an existing valid slice. However, because
// the type is `repr(C)` it can of course be constructed with unsafe rust, or
// even by foreign code. It is the responsibility of _the other code_ to ensure
// that the actual fields are valid for being turned into a slice.

/// A struct for shared "slices" with a stable layout.
///
/// This type is of very little use to purely safe Rust. Instead, the primary
/// value of this type is that it lets you convert a shared slice, `&[T]`, into
/// a stable layout form. This lets you pass it over the C ABI, for example.
///
/// This type has fairly minimal functionality, though there is at least a
/// `Deref` provided. You aren't really intended to store this in a struct or
/// anything. Usually you just turn it back into a normal slice with `into` as
/// soon as you get it.
///
/// ## Unsafety
///
/// Because this type is primarily intended to help _unsafe_ Rust we should
/// discuss the precise guarantees offered:
/// * **Validity Invariants**
///   * The data layout is a [`NonNull<T>`](core::ptr::NonNull) and then a
///     `usize`.
/// * **Soundness Invariants**
///   * The `NonNull<T>` must point to the start of a valid `&[T]`.
///   * The `usize` must be the correct length of that valid `&[T]`.
///   * For as long as the `CSharedSlice` exists the memory in question has a
///     shared borrow over it (tracked via `PhantomData`).
///
/// When you use this type with the C ABI, remember that the C ABI **does not**
/// support generic types. However, if you select a particular type for `T` that
/// is compatible with the C ABI, such as `u8` or `i32`, then that particular
/// monomorphization of `CSharedSlice` will be C ABI compatible as well.
///
/// If you want the pointer value to be nullable, wrap the type in `Option` and
/// you'll get the null-pointer optimization.
#[repr(C)]
pub struct CSharedSlice<'a, T> {
  nn: NonNull<T>,
  len: usize,
  life: PhantomData<&'a [T]>,
}

impl<'a, T: Debug> Debug for CSharedSlice<'a, T> {
  /// Debug prints as a slice would.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    Debug::fmt(self.deref(), f)
  }
}

impl<'a, T> Clone for CSharedSlice<'a, T> {
  #[inline(always)]
  fn clone(&self) -> Self {
    // Note(Lokathor): We can't derive Clone and Copy or CSharedSlice will only
    // be Clone and Copy when T is clone and Copy. However, CSharedSlice is
    // actually a *slice of* T, so it is always Clone and Copy even if T is not.
    *self
  }
}

impl<'a, T> Copy for CSharedSlice<'a, T> { }

impl<'a, T> Default for CSharedSlice<'a, T> {
  /// Defaults to an empty slice.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CSharedSlice<'static, i32> = CSharedSlice::default();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  fn default() -> Self {
    Self::empty_slice()
  }
}

impl<'a, T> Deref for CSharedSlice<'a, T> {
  type Target = [T];
  #[inline(always)]
  fn deref(&self) -> &[T] {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts(self.nn.as_ptr(), self.len) }
  }
}

impl<'a, T> From<&'a [T]> for CSharedSlice<'a, T> {
  fn from(sli: &'a [T]) -> Self {
    let life = PhantomData;
    let len = sli.len();
    // Safety: reference addresses must always be non-null already.
    let nn = unsafe { NonNull::new_unchecked(sli.as_ptr() as *mut T) };
    Self { nn, len, life }
  }
}

impl<'a, T> From<CSharedSlice<'a, T>> for &'a [T] {
  fn from(c_shared: CSharedSlice<'a, T>) -> Self {
    // Safety: See note at the top of the module.
    unsafe { slice::from_raw_parts(c_shared.nn.as_ptr(), c_shared.len) }
  }
}

impl<'a, T> CSharedSlice<'a, T> {
  /// Gives an empty slice as a `const` value.
  ///
  /// ```rust
  /// # use chromium::*;
  /// let c_shared: CSharedSlice<'static, i32> = CSharedSlice::empty_slice();
  /// assert_eq!(c_shared.len(), 0);
  /// ```
  pub const fn empty_slice() -> Self {
    let life = PhantomData;
    let len = 0;
    let nn = NonNull::dangling();
    Self { nn, len, life }
  }
}
