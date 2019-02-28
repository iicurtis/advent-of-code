use num::traits::Signed;
// use std::ops::{Add, Div, Mul, Neg, Sub};
use nom::digit;
use std::ops::Add;

named!(pub signed_digits(&str) -> i64, do_parse!(
        s: recognize!(
            pair!(
                opt!(alt!(tag_s!("+") | tag_s!("-"))),  // maybe sign?
                call!(digit)
            )
        ) >> (s.parse().unwrap())
));

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pt<T: Signed + Copy + Ord = isize>(pub T, pub T);

impl<T: Signed + Copy + Ord> Pt<T> {
    pub fn nn8(self) -> Vec<Self> {
        vec![
            self + Pt::e(),
            self + Pt::ne(),
            self + Pt::n(),
            self + Pt::nw(),
            self + Pt::w(),
            self + Pt::sw(),
            self + Pt::s(),
            self + Pt::se(),
        ]
    }

    pub fn e() -> Self {
        Pt(T::one(), T::zero())
    }
    pub fn ne() -> Self {
        Pt(T::one(), T::one())
    }
    pub fn n() -> Self {
        Pt(T::zero(), T::one())
    }
    pub fn nw() -> Self {
        Pt(-T::one(), T::one())
    }
    pub fn w() -> Self {
        Pt(-T::one(), T::zero())
    }
    pub fn sw() -> Self {
        Pt(-T::one(), -T::one())
    }
    pub fn s() -> Self {
        Pt(T::zero(), -T::one())
    }
    pub fn se() -> Self {
        Pt(T::one(), -T::one())
    }
    pub fn rot90l(self) -> Self {
        Pt(-self.1, self.0)
    }
}

impl<T: Signed + Copy + Ord> Add for Pt<T> {
    type Output = Pt<T>;

    fn add(self, other: Self) -> Self::Output {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a, T: Signed + Copy + Ord> Add<Pt<T>> for &'a Pt<T> {
    type Output = Pt<T>;

    fn add(self, other: Pt<T>) -> Self::Output {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}
