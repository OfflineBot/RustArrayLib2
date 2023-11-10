use std::alloc::{alloc, dealloc, Layout};
use crate::Array2;

impl<T> Array2<T> {
    
    #[allow(unused)]
    pub fn delete(mut self) {
        unsafe { dealloc(self.array as *mut u8, self.layout) };
    }

    #[allow(unused)]
    pub fn clean(&mut self) {
        unsafe { dealloc(self.array as *mut u8, self.layout) };
        let new_layout = Layout::array::<T>(0).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };
        self.array = new_array;
        self.layout = new_layout;
        self.rows = 0;
        self.cols = 0;
    }
}