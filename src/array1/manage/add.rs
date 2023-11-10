use std::alloc::{Layout, alloc};

use crate::Array1;

impl<T> Array1<T> {

    #[allow(unused)]
    pub fn add(&mut self, value: T) {
        let mut size = self.size;
        size += 1;
        let array = self.array;
 
        let new_layout = Layout::array::<T>(size).unwrap();
        let mut new_array = unsafe { alloc(new_layout) as *mut T };

        new_array = self.array;
        unsafe {
            let ptr = new_array.offset((size - 1) as isize);
            std::ptr::write(ptr, value);
        }

        self.array = new_array;
        self.size = size;
        self.layout = new_layout;
    }
}