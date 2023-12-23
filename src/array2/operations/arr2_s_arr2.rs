
use std::ops::{Sub, SubAssign};

use crate::Array2;

impl<T> Sub<Array2<T>> for Array2<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    type Output = Array2<T>;
    fn sub(self, rhs: Array2<T>) -> Array2<T> {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = rhs.rows;
        let cols2 = rhs.cols;

        if cols1 != cols2 {
            panic!("Sizes dont match for Array2 * Array2! {}x{} != {}x{}", rows1, cols1, rows2, cols2);
        }

        let out: Array2<T> = Array2::new(rows1, cols1);

        for i in 0..rows1 {
            for j in 0..cols1 {
                out.set(i, j, self.get(i, j) - rhs.get(i, j));
            }
        }

        out
    }
}
impl<T> SubAssign<Array2<T>> for Array2<T> 
where
    T: Sub<Output = T>,
    T: Clone + Default + Copy
{
    fn sub_assign(&mut self, rhs: Array2<T>) {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = rhs.rows;
        let cols2 = rhs.cols;

        if cols1 != cols2 {
            panic!("Sizes dont match for Array2 * Array2! {}x{} != {}x{}", rows1, cols1, rows2, cols2);
        }

        for i in 0..rows1 {
            for j in 0..cols1 {
                let val = self.get(i, j);
                self.set(i, j, val - rhs.get(i, j));
            }
        }
    }
}


impl<T> Sub<&Array2<T>> for Array2<T> 
where
    T: Sub<Output = T>,
    T: Copy + Default
{
    type Output = Array2<T>;
    fn sub(self, rhs: &Array2<T>) -> Array2<T> {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = rhs.rows;
        let cols2 = rhs.cols;

        if cols1 != cols2 {
            panic!("Sizes dont match for Array2 * Array2! {}x{} != {}x{}", rows1, cols1, rows2, cols2);
        }

        let out: Array2<T> = Array2::new(rows1, cols1);

        for i in 0..rows1 {
            for j in 0..cols1 {
                out.set(i, j, self.get(i, j) - rhs.get(i, j));
            }
        }

        out
    }
}
impl<T> SubAssign<&Array2<T>> for Array2<T> 
where
    T: Sub<Output = T>,
    T: Clone + Default + Copy
{
    fn sub_assign(&mut self, rhs: &Array2<T>) {
        let rows1 = self.rows;
        let cols1 = self.cols;
        let rows2 = rhs.rows;
        let cols2 = rhs.cols;

        if cols1 != cols2 {
            panic!("Sizes dont match for Array2 * Array2! {}x{} != {}x{}", rows1, cols1, rows2, cols2);
        }

        for i in 0..rows1 {
            for j in 0..cols1 {
                let val = self.get(i, j);
                self.set(i, j, val - rhs.get(i, j));
            }
        }
    }
}