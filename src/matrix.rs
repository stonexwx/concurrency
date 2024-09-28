use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T: Debug> fmt::Display for Matrix<T> {
    // display a 2*3 as {1 2 3, 4 5 6}, display a 3*2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }

            if i < self.rows - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: Display + std::fmt::Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix:data:{},rows:{},cols:{}",
            self, self.rows, self.cols
        )?;
        Ok(())
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix size mismatch"));
    }

    let mut data = vec![T::default(); a.rows * b.cols];

    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix::new(data, a.rows, b.cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{:?}", c), "Matrix:data:{7 10,15 22},rows:2,cols:2");
        Ok(())
    }
}
