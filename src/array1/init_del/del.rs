use std::alloc::dealloc;
use crate::Array1;

impl<T> Array1<T> {

    #[allow(unused)]
    pub fn delete(self) {
        unsafe {
            dealloc(self.array as *mut u8, self.layout); 
        } 
    }
}
