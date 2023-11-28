use crate::Array2;

impl<T> Array2<T> {
    #[allow(unused)]
    pub fn get(&self, row: usize, col: usize) -> T {

        if self.rows < row {
            println!("cant get index of Array2! row dont match: self: {} try: {}", self.rows, row);
        }
        if self.cols < col {
            println!(" Cant get index of Array2! col dont match: self: {} try: {}", self.cols, col);
        }

        unsafe {
            let ptr = self.array.offset(row as isize * self.cols as isize + col as isize);
            std::ptr::read(ptr)
        }
    }

    #[allow(unused)]
    pub fn get_raw_array(&self) -> *mut T {
        self.array
    }

    #[allow(unused)]
    pub fn to_vec(&self) -> Vec<Vec<T>> {
        let rows = self.rows;
        let cols = self.cols;

        let mut out: Vec<Vec<T>> = vec![];
        for i in 0..rows {
            let mut slice: Vec<T> = vec![];
            for j in 0..cols {
                slice.push(self.get(i, j));
            }
            out.push(slice);
        }

        out
    }

    #[allow(unused)]
    pub fn size(&self) -> Vec<usize> {
        let vec: Vec<usize> = vec![self.rows, self.cols];
        vec
    }
}