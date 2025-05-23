#![no_std]

use core::fmt::Debug;

pub trait IntoResult: Sized {
    fn into_ok<T>(self) -> Result<Self, T> {
        Ok(self)
    }
    
    fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<T: Sized> IntoResult for T {}

pub trait IntoOption: Sized {
    fn into_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T: Sized> IntoOption for T {}

pub trait AssertEq: Sized + Debug + PartialEq {
    fn assert_eq(self, rhs: Self) {
        assert_eq!(self, rhs);
    }
}

impl<T: Sized + Debug + PartialEq> AssertEq for T {}