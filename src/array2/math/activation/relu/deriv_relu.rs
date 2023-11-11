
use crate::Array2;
use crate::traits::set_one::SetOne;

impl<T> Array2<T> 
where
    T: PartialOrd,
    T: Copy + Default + SetOne
{
    pub fn deriv_relu(&self) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;

        let out: Array2<T> = Array2::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                let val = self.get(i, j);
                match val <= T::default() {
                    true => out.set(i, j, T::default()),
                    false => out.set(i, j, T::set_one()),
                };
            }
        }

        out
    }
}