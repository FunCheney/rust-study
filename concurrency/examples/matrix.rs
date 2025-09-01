use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};
use std::process::Output;
use anyhow::{anyhow, Result};

// 定义一个矩阵的数据结构
#[derive(Debug)]
struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

fn main() -> Result<()> {

    Ok(())
}


fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
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
