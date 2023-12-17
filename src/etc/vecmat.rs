#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::fmt::Display;
use std::iter::Enumerate;
use std::slice::Iter;

use num_traits::int::PrimInt;

use super::coords::Coords2D;

/** A 2D-like structure backed by a Vec */
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VecMat<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct VecMaxIndexedIter<'a, T: Copy, I: PrimInt> {
    _typ: PhantomData<I>,
    iter: Enumerate<Iter<'a, T>>,
    mat: &'a VecMat<T>
}

impl<T: Copy> VecMat<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Self { width, height, data }
    }

    pub fn map_from_str(string: &str, mapper: fn(char) -> T) -> Self {
        let width = string.lines().next().unwrap().len();
        let data: Vec<_> = string.chars().filter(|ch| !ch.is_whitespace()).map(mapper).collect();
        Self{ width, height: data.len() / width, data }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_row(&self, row: usize) -> Vec<T>  {
        assert!(row < self.height());
        self.data[self.width() * row .. self.width() * (row + 1)].to_vec()
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        assert!(col < self.width());
        (0..self.height()).map(|i| self.data[i * self.width() + col]).collect()
    }

    pub fn indexed_iter<I: PrimInt>(&self) -> VecMaxIndexedIter<T, I> {
        VecMaxIndexedIter::new(self)
    }

    pub fn is_in_bounds<I: PrimInt>(&self, pos: Coords2D<I>) -> bool {
        let x = pos.x.to_i64().unwrap();
        let y = pos.y.to_i64().unwrap();

        let bound_x = self.width() as i64;
        let bound_y = self.height() as i64;

        x >= 0 && y >= 0 && x < bound_x && y < bound_y
    }

    pub fn get_or<I: PrimInt + Display>(&self, pos: Coords2D<I>, default: T) -> T {
        if self.is_in_bounds(pos) {
            self[pos]
        } else {
            default
        }
    }

    pub fn rotate_right(&mut self) {
        let mut data = vec![];

        for x in 0..self.width() {
            data.extend(self.get_col(x).into_iter().rev());
        }

        *self = Self { data, width: self.height(), height: self.width() };
    }

    pub fn rotate_left(&mut self) {
        let mut data = vec![];

        for x in (0..self.width()).rev() {
            data.extend(self.get_col(x));
        }

        *self = Self { data, width: self.height(), height: self.width() };
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width(), "x index out of bounds: {x} but width is {}", self.width());
        assert!(y < self.height(), "y index out of bounds: {y} but height is {}", self.height());
        y * self.width + x
    }

    pub fn coords<I: PrimInt>(&self, index: usize) -> Coords2D<I> {
        (I::from(index % self.width).unwrap(), I::from(index / self.width).unwrap()).into()
    }
}

impl VecMat<char> {
    pub fn from_str(string: &str) -> Self {
        let width = string.lines().next().unwrap().len();
        let data: Vec<char> = string.chars().filter(|ch| !ch.is_whitespace()).collect();
        Self{ width, height: data.len() / width, data }
    }
}

impl<T, I> Index<(I, I)> for VecMat<T> 
where T: Copy, 
      I: PrimInt + Display
{
    type Output = T;
    
    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")), 
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &self.data[i]
    }
}

impl<T, I> IndexMut<(I, I)> for VecMat<T>
where T: Copy, 
      I: PrimInt + Display
{
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

impl <'a, T: Copy, I: PrimInt> VecMaxIndexedIter<'a, T, I> {
    pub fn new(mat: &'a VecMat<T>) -> Self {
        let iter = mat.data.iter().enumerate();
        Self { mat, iter, _typ: PhantomData }
    }
}

impl<'a, T: Copy, I: PrimInt> Iterator for VecMaxIndexedIter<'a, T, I> {
    type Item = (Coords2D<I>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, x)| (self.mat.coords(i), *x))
    }
}

impl<T: Copy + Display> Display for VecMat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}