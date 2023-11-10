use crate::Array1;

impl<T> Array1<T>
where
    T: PartialOrd
{
    #[allow(unused)]
    pub fn max(&self) -> T {
        let size = self.size;
        let mut max_size = unsafe { 
            let ptr = self.array.offset(0);
            std::ptr::read(ptr) 
        };
        for i in 1..size {
            let value = unsafe {
                let ptr = self.array.offset(i as isize);
                std::ptr::read(ptr)
            };
            if value > max_size {
                max_size = value;
            }
        }
        max_size
    }

    #[allow(unused)]
    pub fn argmax(&self) -> usize {
        let size = self.size;
        let mut max_size = unsafe {
            let ptr = self.array.offset(0);
            std::ptr::read(ptr)
        };
        let mut max_index = 0;
        
        for i in 1..size {
            let value = unsafe {
                let ptr = self.array.offset(i as isize);
                std::ptr::read(ptr)
            };
            if value > max_size {
                max_size = value;
                max_index = i;
            }
        }

        max_index
    }

}