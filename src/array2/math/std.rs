use std::ops::{AddAssign, Sub, Mul, DivAssign};

use crate::{Array1, Array2};
use crate::traits::{
    sqrt::Sqrt,
    from_usize::FromUsize,
};

impl<T> Array2<T> 
where
    T: AddAssign + DivAssign + Sub<Output = T> + Mul<Output = T>,
    T: Copy + Default + Sqrt + FromUsize
{

    #[allow(unused)]
    pub fn std(&self) -> Array1<T> {
        let out: Array1<T> = Array1::new(self.cols);

        let mut mean_square: Array2<T> = Array2::new(self.cols, self.rows); 
        for i in 0..self.cols {
            let mut sum = T::default();
            for j in 0..self.rows {
                sum += self.get(i, j);
            }
            sum /= T::from_usize(self.rows);
            for j in 0..self.rows {
                let sub_mean = self.get(j, i) - sum;
                let square = sub_mean * sub_mean;
                mean_square.set(j, i, square);
            }
        }

        for i in 0..mean_square.cols {
            let mut sum = T::default();
            for j in 0..mean_square.rows {
                sum += mean_square.get(j, i);
            }
            sum /= T::from_usize(mean_square.rows);
            sum = sum.sqrt();
            out.set_index(i, sum);
        }

        out 
    }
}