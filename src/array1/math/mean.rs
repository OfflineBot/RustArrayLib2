use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign};

use crate::Array1;
use crate::traits::from_usize::FromUsize;

impl<T> Array1<T>
where
    T: AddAssign + DivAssign,
    T: Default + Copy + FromUsize + Debug
{

    /// calculates the mean of Array1
    #[allow(unused)]
    pub fn mean(&self) -> T {

        let size = self.size();
        let mut value = T::default();
        for i in 0..size {
            let item = self.get(i);
            value += item;
        } 
        value /= T::from_usize(size);

        value
    }
}