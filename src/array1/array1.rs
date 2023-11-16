use std::alloc::{Layout, alloc, dealloc};

#[derive(Clone)]
pub struct Array1<T> {
    array: *mut T,
    size: usize,
    layout: Layout,
}

impl<T: Copy> Array1<T> {
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

    #[allow(unused)]
    pub fn clean(&mut self) {
        unsafe { dealloc(self.array as *mut u8, self.layout) };
        let new_layout = Layout::array::<T>(0).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };
        self.array = new_array; 
        self.layout = new_layout;
    }
}