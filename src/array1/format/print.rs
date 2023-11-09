use crate::Array1;

impl<T> std::fmt::Debug for Array1<T>
where
    T: std::fmt::Debug,
    T: Copy + ToString
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut vec: Vec<T> = vec![];

        for i in 0..self.size {
            let value = unsafe {
                *self.array.offset(i as isize)
            };
            vec.push(value);
        }

        let result: String = vec.iter().map(|&f| f.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "Array1 {{ {} }}, size: {}", result, self.size) 
    }
}