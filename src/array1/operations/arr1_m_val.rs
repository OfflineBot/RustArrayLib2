
use std::ops::{Mul, MulAssign};

use crate::Array1;

impl<T> Mul<T> for Array1<T> 
where
    T: Mul<Output = T>,
    T: Copy + Default
{
    type Output = Array1<T>;
    fn mul(self, rhs: T) -> Array1<T> {
        let size1 = self.size;

        let out: Array1<T> = Array1::new(size1);
        for i in 0..size1 {
            out.set_index(i, self.get(i) * rhs);
        }

        out
    }
}
impl<T> MulAssign<T> for Array1<T> 
where
    T: Mul<Output = T>,
    T: Copy + Default
{
    fn mul_assign(&mut self, rhs: T) {
        let size1 = self.size; 

        for i in 0..size1 {
            let val = self.get(i);
            self.set_index(i, val * rhs);
        }
    }
}

impl<T> Mul<&T> for Array1<T> 
where
    T: Mul<Output = T>,
    T: Copy + Default
{
    type Output = Array1<T>;
    fn mul(self, rhs: &T) -> Array1<T> {
        let size1 = self.size;

        let out: Array1<T> = Array1::new(size1);
        for i in 0..size1 {
            out.set_index(i, self.get(i) * rhs.clone());
        }

        out
    }
}
impl<T> MulAssign<&T> for Array1<T> 
where
    T: Mul<Output = T>,
    T: Copy + Default
{
    fn mul_assign(&mut self, rhs: &T) {
        let size1 = self.size; 

        for i in 0..size1 {
            let val = self.get(i);
            self.set_index(i, val * rhs.clone());
        }
    }
}