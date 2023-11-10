use std::alloc::{alloc, Layout};
use crate::Array2;

impl<T> Array2<T> {

    #[allow(unused)]
    pub fn new(rows: usize, cols: usize) -> Array2<T> {

        let layout = Layout::array::<T>(rows * cols).unwrap();  
        let array = unsafe { alloc(layout) as *mut T };

        Array2 {
            array,
            rows,
            cols,
            layout
        }
    }

    #[allow(unused)]
    pub fn new_from_vec(vec: Vec<Vec<T>>) -> Array2<T> {

        let rows = vec.len();
        let cols = vec[0].len();
        let layout = Layout::array::<T>(rows * cols).unwrap();
        let array = unsafe { alloc(layout) as *mut T };

        Array2 {
            array,
            rows,
            cols,
            layout
        }
    }
}