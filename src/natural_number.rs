use std::ops::{Add, Sub};

// -------------------------------------------------------------------------------------------------
// ---- Natural Number and impls -------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

pub trait NaturalNumber:
    Add + Sub + Sized + Copy + Add<Output = Self> + Sub<Output = Self> + PartialEq + Eq
{
    fn to_usize(self) -> usize;
    fn from_usize(u: usize) -> Self;
}

impl NaturalNumber for u16 {
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_usize(u: usize) -> Self {
        u as u16
    }
}

impl NaturalNumber for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_usize(u: usize) -> Self {
        u as u32
    }
}

impl NaturalNumber for u64 {
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_usize(u: usize) -> Self {
        u as u64
    }
}

impl NaturalNumber for usize {
    fn to_usize(self) -> usize {
        self
    }
    fn from_usize(u: usize) -> Self {
        u
    }
}
