use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Mul};
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> std::ops::Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector size mismatch"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
