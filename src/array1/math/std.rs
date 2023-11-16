
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use crate::Array1;
use crate::traits::{
    from_usize::FromUsize,
    sqrt::Sqrt,
};

impl<T> Array1<T>
where
    T: AddAssign + SubAssign + MulAssign + DivAssign,
    T: Default + Clone + Sqrt + FromUsize
{
    /// calculate the standard deviation of Array1
    #[allow(unused)]
    pub fn std(&self) -> T {

        let size = self.size();
        let tsize = T::from_usize(size);

        let mut mean = T::default();
        for i in 0..size {
            let value = self.get(i);
            mean += value;
        }
        mean /= tsize.clone();

        let mut mean2 = T::default();
        for i in 0..size {
            let mut sub_item = self.get(i);
            sub_item -= mean.clone();
            sub_item *= sub_item.clone();
            mean2 += sub_item;
        } 
        mean2 /= tsize;
        mean2.sqrt()
        
    }
}