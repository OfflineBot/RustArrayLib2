
use std::ops::{Sub, SubAssign};

use crate::Array1;

impl<T> Sub<Array1<T>> for Array1<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    type Output = Array1<T>;
    fn sub(self, rhs: Array1<T>) -> Array1<T> {
        let size1 = self.size;
        let size2 = rhs.size;  

        if size1 != size2 {
            panic!("Sizes dont match for Array1 - Array1! {} != {}", size1, size2);
        }

        let out: Array1<T> = Array1::new(size1);

        for i in 0..size1 {
            out.set_index(i, self.get(i) - rhs.get(i));
        }

        out
    }
}
impl<T> SubAssign<Array1<T>> for Array1<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    fn sub_assign(&mut self, rhs: Array1<T>) {
        let size1 = self.size;
        let size2 = rhs.size;

        if size1 != size2 {
            panic!("Sizes dont match for Array1 - Array1! {} != {}", size1, size2);
        }
        
        for i in 0..size1 {
            let val = self.get(i);
            self.set_index(i, val - rhs.get(i));
        }
    }
}

impl<T> Sub<&Array1<T>> for Array1<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    type Output = Array1<T>;
    fn sub(self, rhs: &Array1<T>) -> Array1<T> {
        let size1 = self.size;
        let size2 = rhs.size;  

        if size1 != size2 {
            panic!("Sizes dont match for Array1 - Array1! {} != {}", size1, size2);
        }

        let out: Array1<T> = Array1::new(size1);

        for i in 0..size1 {
            out.set_index(i, self.get(i) - rhs.get(i));
        }

        out
    }
}
impl<T> SubAssign<&Array1<T>> for Array1<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    fn sub_assign(&mut self, rhs: &Array1<T>) {
        let size1 = self.size;
        let size2 = rhs.size;

        if size1 != size2 {
            panic!("Sizes dont match for Array1 - Array1! {} != {}", size1, size2);
        }
        
        for i in 0..size1 {
            let val = self.get(i);
            self.set_index(i, val - rhs.get(i));
        }
    }
}