
use crate::Array2;

impl<T> Array2<T> 
where
    T: Copy + Default
{
    pub fn t(&self) -> Array2<T> {
        let rows = self.rows;
        let cols = self.cols;

        let out: Array2<T> = Array2::new(cols, rows);

        for i in 0..rows {
            for j in 0..cols {
                out.set(j, i, self.get(i, j));
            }
        }

        out
    }
}