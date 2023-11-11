
use std::ops::{Add, Sub, Mul, Div};

use crate::Array2;

impl<T> Add<&Array2<T>> for Array2<T> 
where
    T: Add<Output = T>,
    T: Copy
{
    type Output = Array2<T>;

    fn add(self, other: &Array2<T>) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;

        let mut out: Array2<T> = Array2::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j) + other.get(i, j);
                out.set(i, j, val);
            }
        }

        out
    }
}

impl<T> Sub<&Array2<T>> for Array2<T> 
where
    T: Sub<Output = T>,
    T: Copy
{
    type Output = Array2<T>;

    fn sub(self, other: &Array2<T>) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;  

        let mut out: Array2<T> = Array2::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j) - other.get(i, j);
                out.set(i, j, val);
            }
        }

        out
    }
}

impl<T> Mul<&Array2<T>> for Array2<T> 
where
    T: Mul<Output = T>,
    T: Copy
{
    type Output = Array2<T>;

    fn mul(self, other: &Array2<T>) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;  

        let mut out: Array2<T> = Array2::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j) * other.get(i, j);
                out.set(i, j, val);
            }
        }

        out
    }
}

impl<T> Div<&Array2<T>> for Array2<T> 
where
    T: Div<Output = T>,
    T: Copy
{
    type Output = Array2<T>;

    fn div(self, other: &Array2<T>) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;  

        let mut out: Array2<T> = Array2::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j) / other.get(i, j);
                out.set(i, j, val);
            }
        }

        out
    }
}