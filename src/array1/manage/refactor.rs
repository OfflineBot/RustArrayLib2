
use crate::Array1;

impl<T> Array1<T>
where
    T: PartialOrd,
    T: Default + Copy
{
    pub fn replace_zero(self, e_minus: T) {
        let size = self.size;

        for i in 0..size {
            let value = self.get(i);
            if value == T::default() {
                self.set_index(i, e_minus);
            }
        }
    }
}