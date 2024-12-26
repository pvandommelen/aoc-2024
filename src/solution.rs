// Copied and extended from https://github.com/agubelu/AoC-rust-template/tree/master

use Solution::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
    Nothing(),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolutionTuple(pub Solution, pub Solution);

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I8(x) => x.fmt(f),
            I16(x) => x.fmt(f),
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            Isize(x) => x.fmt(f),
            U8(x) => x.fmt(f),
            U16(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            Usize(x) => x.fmt(f),
            Str(x) => x.fmt(f),
            Nothing() => f.write_str("-"),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Str(sol.to_owned())
    }
}

impl<A, B> From<(A, B)> for SolutionTuple
where
    A: Into<Solution>,
    B: Into<Solution>,
{
    fn from(value: (A, B)) -> Self {
        SolutionTuple(value.0.into(), value.1.into())
    }
}

impl<A> From<(A, ())> for SolutionTuple
where
    A: Into<Solution>,
{
    fn from(value: (A, ())) -> Self {
        SolutionTuple(value.0.into(), Nothing())
    }
}

impl From<((), ())> for SolutionTuple {
    fn from(_: ((), ())) -> Self {
        SolutionTuple(Nothing(), Nothing())
    }
}
