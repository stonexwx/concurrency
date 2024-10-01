use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use anyhow::{anyhow, Result};

use crate::{vector::dot_product, Vector};

const NUM_THREADS: usize = 4;

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

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiplication failed")
    }
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix size mismatch"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    let matrix_len: usize = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new((a.data[i * a.cols..(i + 1) * a.cols]).to_vec());
            let col = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.cols)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {}", e);
            };
            let ret = rx.recv()?;
            receivers.push(ret);
        }
    }

    for ret in receivers {
        data[ret.idx] = ret.value;
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
        let c = a * b;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{:?}", c), "Matrix:data:{7 10,15 22},rows:2,cols:2");
        Ok(())
    }
}
