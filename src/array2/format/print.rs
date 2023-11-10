use crate::Array2;

impl<T> std::fmt::Debug for Array2<T>
where 
    T: std::fmt::Debug,
    T: Copy + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut vec: Vec<Vec<T>> = vec![];

        for i in 0..self.rows {
            let mut slice: Vec<T> = vec![];
            for k in 0..self.cols {
                let value = unsafe {
                    let ptr = self.array.offset(i as isize * self.cols as isize + k as isize);
                    std::ptr::read(ptr)
                };
                slice.push(value);
            }
            vec.push(slice);
        }

        let mut end_str: String = format!("{:?}", vec); 
        end_str = end_str.replace("[", "{").replace("]", "}");

        write!(f, "{}, size: {}x{}", end_str, self.rows, self.cols)
    }
}