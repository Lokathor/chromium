#![allow(bad_style)]

use core::ops::{Deref, DerefMut};

use chromium::*;

#[test]
fn test_SharedSlice_to_from() {
  let my_slice: &[i32] = &[1, 2, 3, 4];
  let c = SharedSlice::from(my_slice);
  assert_eq!(c.deref(), &[1, 2, 3, 4]);
  let back_as_a_slice: &[i32] = c.into();
  assert_eq!(back_as_a_slice, &[1, 2, 3, 4]);
}

#[test]
fn test_UniqueSlice_to_from() {
  let my_slice: &mut [i32] = &mut [1, 2, 3, 4];
  let mut c = UniqueSlice::from(my_slice);
  assert_eq!(c.deref_mut(), &mut [1, 2, 3, 4]);
  let back_as_a_slice: &mut [i32] = c.into();
  assert_eq!(back_as_a_slice, &mut [1, 2, 3, 4]);
}

#[test]
fn test_SharedStr_to_from() {
  let my_str: &str = "hello";
  let s = SharedStr::from(my_str);
  assert_eq!(s.deref(), "hello");
  let back_as_a_str: &str = s.into();
  assert_eq!(back_as_a_str, "hello");
}

#[test]
fn test_UniqueStr_to_from() {
  let mut hello = String::from("hello");
  let my_str: &mut str = hello.deref_mut();
  let mut s = UniqueStr::from(my_str);
  assert_eq!(s.deref_mut(), "hello");
  let back_as_a_str: &mut str = s.into();
  assert_eq!(back_as_a_str, "hello");
}
