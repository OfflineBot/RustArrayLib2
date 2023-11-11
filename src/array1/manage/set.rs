use crate::Array1;

impl<T: Copy> Array1<T> {

    #[allow(unused)]
    pub fn set_index(&self, index: usize, value: T) {
       
        if index >= self.size {
            panic!("Sizes dont match!");
        }
        unsafe { 
            let ptr = self.array.offset(index as isize);
            std::ptr::write(ptr, value); 
        };
    }

    #[allow(unused)]
    pub fn set_vec(&self, vec: Vec<T>) {
        let size = self.size;
        let vec_size = vec.len();
        
        if size != vec_size {
            panic!("Sizes dont match!");
        }

        for i in 0..size {
            unsafe {
                let ptr = self.array.offset(i as isize);
                std::ptr::write(ptr, vec[i]);
            }
        }

    }
}
