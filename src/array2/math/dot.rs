use std::{alloc::{alloc, Layout}, ops::{AddAssign, Mul}};

use crate::Array2;

impl<T> Array2<T>
where
    T: AddAssign<T> + Mul
{
    pub fn dot(&self, other: &Array2<T>) -> Array2<T> {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = other.rows;
        let cols2 = other.cols;

        let layout = Layout::array::<T>(rows1 * cols2).unwrap();
        let array = unsafe { alloc(layout) as *mut T };

        for i in 0..rows1 {
            for j in 0..cols2 {
                for k in 0..cols1 {
                    unsafe {
                        let ptr = array.offset(i as isize * cols2 as isize + j as isize);
                        let ptr_1 = self.array.offset(i as isize * cols1 as isize + k as isize);
                        let ptr_2 = other.array.offset(k as isize * cols2 as isize + j as isize);
                        let ptr_var = std::ptr::read(ptr);
                        let ptr_1_var = std::ptr::read(ptr_1);
                        let ptr_2_var = std::ptr::read(ptr_2);
                        
                        let out = ptr_var + (ptr_1_var * ptr_2_var);
                        std::ptr::write(ptr, out);

                    }
                }
            }
        }

        Array2 {
            array,
            rows: rows1,
            cols: cols2,
            layout
        }
    } 
}