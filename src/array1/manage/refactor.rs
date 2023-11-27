
use crate::Array1;

impl<T> Array1<T>
where
    T: PartialOrd,
    T: Default + Copy
{

    ///
    /// replaces all 0/0.0 in Array1 with the value `e_minus`
    /// 
    /// ## Examples:
    /// ```
    /// let x: Vec<f32> = vec![1.0, 3.2, 0.0];
    /// let y: Array1<f32> = Array1::from_vec(x);
    /// y.replace_zero(0.1) = [1.0, 3.2, 0.1];
    /// ```
    #[allow(unused)]
    pub fn replace_zero(mut self, e_minus: T) {
        let size = self.size;

        for i in 0..size {
            let value = self.get(i);
            if value == T::default() {
                self.set_index(i, e_minus);
            }
        }
    }
}