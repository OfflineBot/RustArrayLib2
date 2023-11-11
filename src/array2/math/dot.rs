use std::ops::{
    Mul, Add, AddAssign
};

use crate::Array2;

impl<T> Array2<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign,
    T: Default + Copy
{
    pub fn dot(&self, other: &Array2<T>) -> Array2<T> {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = other.rows;
        let cols2 = other.cols;

        if cols1 != rows2 {
            panic!("Matricies dont match for matrix multiplication!");
        }

        let out_array: Array2<T> = Array2::new(rows1, cols2);

        for i in 0..rows1 {
            for j in 0..cols2 {
                let mut sum = T::default();
                for k in 0..cols1 {
                    sum += self.get(i, k) * other.get(k, j);
                }
                out_array.set(i, j, sum);
            }
        }

        out_array 
    } 
}