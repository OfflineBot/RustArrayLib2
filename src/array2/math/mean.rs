use std::{
    alloc::Layout, 
    ops::{AddAssign, DivAssign},
};

use crate::{Array1, Array2};
use crate::traits::from_usize::FromUsize;

impl<T> Array2<T>
where 
    T: AddAssign + DivAssign,
    T: Copy + Default + FromUsize<T>
{
    
    #[allow(unused)]
    pub fn mean(&self) -> Array1<T> {

        let mut out: Array1<T> = Array1::new(self.cols);

        let size = self.cols;
        let layout = Layout::array::<T>(size).unwrap();

        for i in 0..self.cols {
            let mut sum = T::default();
            for j in 0..self.rows {
                sum += self.get(i, j);
            }
            sum /= T::from_usize(self.rows);
            out.set_index(i, sum);
        }

        out
    }
}