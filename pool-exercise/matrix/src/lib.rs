use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Copy + 
    PartialEq + 
    Sized 
{
    type Item;
}

impl Scalar for u32 {
    type Item = u32;
}

impl Scalar for i32 {
    type Item = i32;
}

impl Scalar for f64 {
    type Item = f64;
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + From<u8>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::from(0)]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::from(0); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::from(0); n]; n];
        for i in 0..n {
            matrix[i][i] = T::from(1);
        }
        Matrix(matrix)
    }
}