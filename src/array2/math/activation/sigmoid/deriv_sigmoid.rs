

use std::ops::{Add, Sub, Mul, Div};

use crate::Array2;
use crate::traits::{
    set_one::SetOne,
    exp::Exp,
};

impl<T> Array2<T> 
where
    T: Sub<Output = T> + Add<Output = T> + Div<Output = T> + Mul<Output = T>,
    T: Copy + SetOne + Exp + Default
{
    pub fn deriv_sigmoid(&self) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;

        let mut out: Array2<T> = Array2::new(rows, cols);
        let mut sig = self.to_owned();
        sig = sig.sigmoid();

        for i in 0..rows {
            for j in 0..cols {
                let val = sig.get(i, j) * (T::set_one() - sig.get(i, j));
                out.set(i, j, val);
            }
        }

        out
    }
}