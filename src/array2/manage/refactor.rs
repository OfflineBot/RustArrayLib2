
use crate::Array2;

impl<T> Array2<T>
where
    T: PartialOrd,
    T: Copy + Default
{
    pub fn replace_zero(&self, e_minus: T) {
        let row = self.rows;
        let col = self.cols;

        for i in 0..row {
            for j in 0..col {
                let value = self.get(i, j);
                if value == T::default() {
                    self.set(i, j, e_minus);
                }
            }
        }
    }
}