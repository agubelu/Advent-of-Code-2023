use std::ops::{Add, Sub, AddAssign, SubAssign, Mul};
use num_traits::int::PrimInt;
use num_traits::sign::Signed;

/** A pair of integers representing 2D coordinates. */
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coords2D<T: PrimInt> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt> Coords2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        (T::zero(), T::zero()).into()
    }
}

impl<T: PrimInt + Signed> Coords2D<T> {
    pub fn up() -> Self {
        (T::zero(), -T::one()).into()
    }

    pub fn down() -> Self {
        (T::zero(), T::one()).into()
    }

    pub fn left() -> Self {
        (-T::one(), T::zero()).into()
    }

    pub fn right() -> Self {
        (T::one(), T::zero()).into()
    }

    pub fn manhattan_dist(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T: PrimInt> From<(T, T)> for Coords2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: PrimInt> From<Coords2D<T>> for (T, T) {
    fn from(c: Coords2D<T>) -> (T, T) {
        (c.x, c.y)
    }
}

impl <T: PrimInt> Add<Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: PrimInt> AddAssign<Coords2D<T>> for Coords2D<T> {
    fn add_assign(&mut self, rhs: Coords2D<T>) {
        *self = *self + rhs;
    }
}

impl <T: PrimInt> Sub<Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl <T: PrimInt> SubAssign<Coords2D<T>> for Coords2D<T> {
    fn sub_assign(&mut self, rhs: Coords2D<T>) {
        *self = *self - rhs;
    }
}

impl <T: PrimInt> Add<&Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: PrimInt> Sub<&Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl <T: PrimInt> Mul<T> for Coords2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
