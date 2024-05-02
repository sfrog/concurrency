use anyhow::{anyhow, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix dimensions mismatch: a.col != b.row"));
    }

    let mut data = vec![T::default(); a.row * b.col];

    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    let result = Matrix {
        data,
        row: a.row,
        col: b.col,
    };

    Ok(result)
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display as {1 2 3, 4 5 6, 7 8 9}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j < self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    // debug as Matrix { row: 3, col: 3, data: [1, 2, 3, 4, 5, 6, 7, 8, 9] }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix {{ row: {}, col: {}, data: {} }}",
            self.row, self.col, self
        )
    }
}

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix { row: 2, col: 2, data: {22 28, 49 64} }"
        );

        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
        Ok(())
    }
}
