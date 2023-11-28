use crate::Array1;

impl<T> Array1<T> {

    ///
    /// get singe value from index
    /// 
    /// ## Example:
    /// ``` 
    /// let x: Array1<f32> = Array1::new(2);
    /// x.set_index(0, 3.2);
    /// x.get(0) = 3.2
    /// x.get(1) = 0.0 
    /// ``` 
    #[allow(unused)]
    pub fn get(&self, index: usize) -> T {
        if self.size <= index {
            panic!("Cant get index {} from total length {}", index, self.size);
        } 
        unsafe {
            let ptr = self.array.offset(index as isize);
            std::ptr::read(ptr)
        }
    }
    
    #[allow(unused)]
    pub fn get_raw_array(&self) -> *mut T {
        self.array
    }

    #[allow(unused)]
    pub fn to_vec(&self) -> Vec<T> {
        let size = self.size;
        let array = self.array;
        let mut vec: Vec<T> = vec![];

        for i in 0..size {
            let value = unsafe { 
                let data = array.offset(i as isize);
                std::ptr::read(data)
            };
            vec.push(value);
        }

        vec
    }

    #[allow(unused)]
    pub fn size(&self) -> usize {
        self.size
    }
}