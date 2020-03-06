#![allow(bad_style)]

use core::ops::{Deref, DerefMut};

use chromium::*;

#[test]
fn test_CSharedSlice_to_from() {
  let my_slice: &[i32] = &[1, 2, 3, 4];
  let c = CSharedSlice::from(my_slice);
  assert_eq!(c.deref(), &[1,2,3,4]);
  let back_as_a_slice: &[i32] = c.into();
  assert_eq!(back_as_a_slice, &[1,2,3,4]);
}

#[test]
fn test_CUniqueSlice_to_from() {
  let my_slice: &mut [i32] = &mut [1, 2, 3, 4];
  let mut c = CUniqueSlice::from(my_slice);
  assert_eq!(c.deref_mut(), &mut [1,2,3,4]);
  let back_as_a_slice: &mut [i32] = c.into();
  assert_eq!(back_as_a_slice, &mut [1,2,3,4]);
}
