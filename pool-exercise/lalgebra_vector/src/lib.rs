use std::fmt::{Debug, Formatter, Result as FmtResult};
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
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

#[derive(PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar + std::fmt::Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Vector({:?})", self.0)
    }
}

impl<T: Scalar> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector(self.0.clone())
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            None
        } else {
            Some(Vector(
                self.0.iter()
                    .zip(other.0.iter())
                    .map(|(a, b)| *a + *b)
                    .collect()
            ))
        }
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            None
        } else {
            Some(
                self.0.iter()
                    .zip(other.0.iter())
                    .fold(T::zero(), |acc, (a, b)| acc + *a * *b)
            )
        }
    }
}