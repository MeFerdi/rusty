use std::ops::Mul;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() || self.0[0].is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + std::iter::Sum + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        let lhs_rows = self.number_of_rows();
        let lhs_cols = self.number_of_cols();
        let rhs_rows = other.number_of_rows();
        let rhs_cols = other.number_of_cols();

        if lhs_cols != rhs_rows {
            return None;
        }

        let mut output_data = vec![vec![T::default(); rhs_cols]; lhs_rows];

        for row_idx in 0..lhs_rows {
            for col_idx in 0..rhs_cols {
                output_data[row_idx][col_idx] = (0..lhs_cols)
                    .map(|inner_idx| self.0[row_idx][inner_idx] * other.0[inner_idx][col_idx])
                    .sum();
            }
        }

        Some(Matrix(output_data))
    }
}