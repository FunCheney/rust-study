use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};
use anyhow::{anyhow, Result};

// 定义一个矩阵的数据结构
#[derive(Debug)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}


pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where T: Debug + Add<Output = T> + AddAssign + Mul<Output = T> + Copy {

    if a.cols != b.rows {
        return  Err(anyhow!("Matrix dimensions do not match"));
    }

    let mut data = Vec::with_capacity(a.rows * a.cols);

    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix{
        data: data,
        rows: a.rows,
        cols: b.cols
    })

}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl  Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[1, 2, 3, 4, 5, 6], 3, 2);
        let result = multiply(&a, &b)?;
        assert_eq!(format!("{:?}", result), "Matrix {rows: 2, cols: 3 , {{22 28, 49, 65}}}");
        Ok(())
    }

}