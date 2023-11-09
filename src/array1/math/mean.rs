use std::ops::{AddAssign, DivAssign};

use crate::Array1;

impl<T> Array1<T>
where
    T: AddAssign + DivAssign<usize>,
    T: Default + Copy
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
            value /= size;
        } 

        value
    }
}