use crate::Array1;

impl<T> Array1<T> 
where
    T: Clone + Copy
{

    ///
    /// set `value` on the `index` of the Array1
    /// 
    /// ## Examples:
    /// ```
    /// let x: Array1<f32> = Array1::new(2);
    /// x.set_index(1, 3.2) = [0.0, 3.2]
    /// ```
    #[allow(unused)]
    pub fn set_index(&self, index: usize, value: T) {
       
        if index > self.size() {
            panic!("Sizes dont match!");
        }
        unsafe { 
            let ptr = self.array.offset(index as isize);
            std::ptr::write(ptr, value); 
        };
    }

    ///
    /// replaces the values of Array1 with `vec`
    /// Vector must be same length than Array1
    /// 
    /// ## Examples:
    /// ```
    /// let x: Array1<f32> = Array1::new(2); 
    /// x.set_vec(vec![2.0, 1.4]) = [2.0, 1.4]
    /// x.set_vec(vec![2.0]) ! vec_size != Array1_size
    /// ```
    #[allow(unused)]
    pub fn set_vec(&self, vec: Vec<T>) {
        let size = self.size();
        let vec_size = vec.len();
        
        if size != vec_size {
            panic!("Sizes dont match!");
        }

        for i in 0..size {
            self.set_index(i, vec[i]);
        }

    }
}
