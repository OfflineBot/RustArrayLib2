
use std::ops::AddAssign;

use crate::{Array1, Array2};

impl<T> Array2<T> 
where
    T: AddAssign,
    T: Copy + Default
{
    pub fn sum(&self) -> Array1<T> {
        let rows = self.rows;
        let cols = self.cols;
        let out: Array1<T> = Array1::new(cols);

        for i in 0..cols {
            let mut sum = T::default();
            for j in 0..rows {
                let value = self.get(j, i);
                sum += value;
            }
            out.set_index(i, sum); 
        }

        out
    }
}