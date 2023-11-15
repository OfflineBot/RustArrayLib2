use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign};

use crate::Array1;
use crate::traits::from_usize::FromUsize;

impl<T> Array1<T>
where
    T: AddAssign + DivAssign,
    T: Default + Copy + FromUsize + Debug
{

    #[allow(unused)]
    pub fn mean(&self) -> T {

        let size = self.size;
        let mut value = T::default();
        for i in 0..size {
            unsafe {
                let ptr = self.array.offset(i as isize);
                let item = std::ptr::read(ptr);
                value += item;
            }
        } 
        value /= T::from_usize(size);

        value
    }
}