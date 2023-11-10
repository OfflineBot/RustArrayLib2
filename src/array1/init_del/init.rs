use std::alloc::{Layout, alloc};
use crate::Array1;

impl<T: Copy> Array1<T> {
    #[allow(unused)]
    pub fn new(size: usize) -> Self {
        let layout = Layout::array::<T>(size).unwrap(); 
        let array = unsafe { alloc(layout) as *mut T };
        Array1 {
            array,
            size,
            layout
        }
    }

    #[allow(unused)]
    pub fn new_from_vec(vec: Vec<T>) -> Self {
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