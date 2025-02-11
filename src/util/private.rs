use crate::reader::{SliceRead, UncheckedSliceRead};
use bytes::Bytes;
use faststr::FastStr;

// Prevent users from implementing the trait in sonic-rs.
pub trait Sealed {}
impl Sealed for usize {}
impl Sealed for str {}
impl Sealed for std::string::String {}
impl Sealed for FastStr {}
impl Sealed for Bytes {}
impl Sealed for u8 {}
impl<'de> Sealed for SliceRead<'de> {}
impl<'de> Sealed for UncheckedSliceRead<'de> {}
impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
impl<T> Sealed for [T] where T: Sized + Sealed {}
