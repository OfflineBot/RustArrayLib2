use std::alloc::{Layout, alloc};

use crate::Array1;

impl<T: Copy> Array1<T> {


    /// Create empty Array1 with given `size`
    /// 
    ///  ## Examples:
    ///  ```
    ///  vec![0.0, 0.0] = Array1::new(2)
    /// ```
    #[allow(unused)]
    pub fn new(size: usize) -> Self {
        let layout = Layout::array::<T>(size).unwrap(); 
        let array = unsafe { alloc(layout) as *mut T };
        let out: Array1<T>;
        Array1 {
            array,
            size,
            layout
        }
    }

    ///
    /// Create Array1 from Vec<T>
    /// ## Examples: 
    /// ```
    /// let x: Vec<f32> = vec![1.0, 2.0];
    /// let y: Array1<f32> = Array1::from_vec(x);
    /// -> x = y 
    /// ```
    /// 
    #[allow(unused)]
    pub fn from_vec(vec: Vec<T>) -> Self {
        let size = vec.len();
        let layout = Layout::array::<T>(size).unwrap();
        let array = unsafe { alloc(layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr = array.offset(i as isize);
                std::ptr::write(ptr, vec[i]);
            }
        }

        Array1 {
            array, 
            size,
            layout
        }
    }
}