use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::new();
        for (row1, row2) in self.0.iter().zip(other.0.iter()) {
            if row1.len() != row2.len() {
                return None;
            }
            let mut new_row = Vec::new();
            for (a, b) in row1.iter().zip(row2.iter()) {
                new_row.push(*a + *b);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::new();
        for (row1, row2) in self.0.iter().zip(other.0.iter()) {
            if row1.len() != row2.len() {
                return None;
            }
            let mut new_row = Vec::new();
            for (a, b) in row1.iter().zip(row2.iter()) {
                new_row.push(*a - *b);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}