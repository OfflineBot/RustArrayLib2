
use std::ops::{Sub, Add, Div};

use crate::Array2;
use crate::traits::{
    exp::Exp,
    set_one::SetOne
};

impl<T> Array2<T> 
where
    T: Sub<Output = T> + Add<Output = T> + Div<Output = T>,
    T: Copy + SetOne + Exp + Default
{
    pub fn sigmoid(&self) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;

        let out: Array2<T> = Array2::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j);
                let diff = T::default() - val; 
                let sig = T::set_one() / (T::set_one() + diff.exp());
                out.set(i, j, sig);
            }
        }

        out
    }
}