use std::alloc::{alloc, dealloc, Layout};

#[allow(unused)]
pub struct Array1<T> {
    pub array: *mut T,
    pub size: usize,
    pub layout: Layout,
}


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
                *array.offset(i as isize) = vec[i];
            }
        }

        Array1 {
            array, 
            size,
            layout
        }
    }

    #[allow(unused)]
    pub fn delete(self) {
        unsafe {
            dealloc(self.array as *mut u8, self.layout); 
        } 
    }

}
