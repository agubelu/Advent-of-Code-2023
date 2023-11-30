use std::ops::{Index, IndexMut};
use std::fmt::Display;

use num_traits::int::PrimInt;

use super::coords::Coords2D;

/** A 2D-like structure backed by a Vec */
#[derive(Clone, Debug)]
pub struct VecMat<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy> VecMat<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        Self { width, height, data }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    ////////////////////////////////////////////////////////////////////////////

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width(), "x index out of bounds: {} but width is {}", x, self.width());
        assert!(y < self.height(), "y index out of bounds: {} but height is {}", y, self.height());
        y * self.width + x
    }
}

impl<T: Copy, I: PrimInt + Display> Index<(I, I)> for VecMat<T> {
    type Output = T;
    
    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")), 
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &self.data[i]
    }
}

impl<T: Copy, I: PrimInt + Display> IndexMut<(I, I)> for VecMat<T> {
    fn index_mut(&mut self, (x, y): (I, I)) -> &mut Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")), 
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &mut self.data[i]
    }
}

impl<T, I> Index<Coords2D<I>> for VecMat<T>
where T: Copy, 
      I: PrimInt + Display
{
    type Output = T;

    fn index(&self, Coords2D { x, y }: Coords2D<I>) -> &Self::Output {
        &self[(x, y)]
    }
}

impl<T, I> IndexMut<Coords2D<I>> for VecMat<T>
where T: Copy, 
      I: PrimInt + Display
{

    fn index_mut(&mut self, Coords2D { x, y }: Coords2D<I>) -> &mut Self::Output {
        &mut self[(x, y)]
    }
}