use crate::Array2;

impl<T> Array2<T> {

    #[allow(unused)]
    pub fn set(&self, rows: usize, cols: usize, value: T) {
        if self.rows <= rows || self.cols <= cols {
            panic!("cant set the value! index is to high: {}x{} > {}x{}", rows, cols, self.rows, self.cols);
        }
        unsafe {
            let ptr = self.array.offset(rows as isize * self.cols as isize + cols as isize);
            std::ptr::write(ptr, value);
        }
    }
}