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

        let mean_squared: Array2<T> = Array2::new(self.rows, self.cols);
        for i in 0..self.cols {
            let mut sum = T::default();

            for j in 0..self.rows {
                let value = self.get(j, i);
                sum += value;
            }
            sum /= T::from_usize(self.rows);

            for j in 0..self.rows {
                let minus_mean = self.get(j, i) - sum; 
                let squared = minus_mean * minus_mean;
                mean_squared.set(j, i, squared);
            }
        }

        for i in 0..self.cols {
            let mut sum = T::default();
            for j in 0..self.rows {
                let value = mean_squared.get(j, i);
                sum += value; 
            }
            sum /= T::from_usize(self.rows);
            let sqrt = sum.sqrt();
            out.set_index(i, sqrt);
        }

        out 
    }
}