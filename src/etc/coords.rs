use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, Neg};
use num_traits::{Float, Num, PrimInt, Signed};

/** A pair of numbers representing 2D coordinates. */
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coords2D<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Coords2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        (T::zero(), T::zero()).into()
    }
}

impl<T: Num + Signed + Copy> Coords2D<T> {
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

    pub fn go_up(&self) -> Self {
        self + Self::up()
    }

    pub fn go_down(&self) -> Self {
        self + Self::down()
    }

    pub fn go_left(&self) -> Self {
        self + Self::left()
    }

    pub fn go_right(&self) -> Self {
        self + Self::right()
    }

    pub fn neighbors(&self) -> [Self; 4] {
        [self + Self::up(), self + Self::down(), self + Self::left(), self + Self::right()]
    }

    pub fn neighbors_diag(&self) -> [Self; 8] {
        [self + Self::up(), self + Self::down(), self + Self::left(), self + Self::right(),
         self + Self::up() + Self::left(), self + Self::up() + Self::right(),
         self + Self::down() + Self::left(), self + Self::down() + Self::right(),]
    }
}

impl<T: PrimInt + Signed> Coords2D<T> {
    pub fn manhattan_dist(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T: Float + Signed> Coords2D<T> {
    pub fn euclidean_dist(&self, other: &Self) -> T {
        let sqs = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        sqs.sqrt()
    }
}

impl<T: Num> From<(T, T)> for Coords2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Num> From<Coords2D<T>> for (T, T) {
    fn from(c: Coords2D<T>) -> (T, T) {
        (c.x, c.y)
    }
}

impl <T: Num> Add<Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: Num + Copy> Add<Coords2D<T>> for &Coords2D<T> {
    type Output = Coords2D<T>;

    fn add(self, rhs: Coords2D<T>) -> Self::Output {
        Coords2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: Num + Copy> AddAssign<Coords2D<T>> for Coords2D<T> {
    fn add_assign(&mut self, rhs: Coords2D<T>) {
        *self = *self + rhs;
    }
}

impl <T: Num> Sub<Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl <T: Num + Copy> SubAssign<Coords2D<T>> for Coords2D<T> {
    fn sub_assign(&mut self, rhs: Coords2D<T>) {
        *self = *self - rhs;
    }
}

impl <T: Num + Copy> Add<&Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl <T: Num + Copy> Sub<&Coords2D<T>> for Coords2D<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl <T: Num + Copy> Mul<T> for Coords2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl <T: Num + Neg<Output = T>> Neg for Coords2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
