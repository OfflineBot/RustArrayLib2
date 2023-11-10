use crate::Array2;

impl<T> Array2<T> {

    #[allow(unused)]
    pub fn set(&mut self, rows: usize, cols: usize, value: T) {
        unsafe {
            let ptr = self.array.offset(rows as isize * self.cols as isize + cols as isize);
            std::ptr::write(ptr, value);
        }
    }
}