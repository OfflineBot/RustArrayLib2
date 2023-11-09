
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use crate::Array1;


pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait FromUsize<T> {
    fn from_usize(value: usize) -> T;
}

impl FromUsize<f32> for f32 {
    fn from_usize(value: usize) -> f32 {
        value as f32
    }
}

impl FromUsize<f64> for f64 {
    fn from_usize(value: usize) -> f64 {
        value as f64
    }
}

impl<T> Array1<T>
where
    T: AddAssign + SubAssign + MulAssign + DivAssign,
    T: Default + Clone + Sqrt + FromUsize<T>
{

    #[allow(unused)]
    pub fn std(&self) -> T {

        let size = self.size;
        let tsize = T::from_usize(size);

        let mut mean = T::default();
        for i in 0..size {
            let value = unsafe {
                let ptr = self.array.offset(i as isize);
                std::ptr::read(ptr)
            };
            mean += value;
        }
        mean /= tsize.clone();

        let mut mean2 = T::default();
        for i in 0..size {
            let mut sub_item = unsafe {
                let ptr = self.array.offset(i as isize);
                std::ptr::read(ptr)
            };
            sub_item -= mean.clone();
            sub_item *= sub_item.clone();
            mean2 += sub_item;
        } 
        mean2 /= tsize;
        mean2.sqrt()
        
    }
}