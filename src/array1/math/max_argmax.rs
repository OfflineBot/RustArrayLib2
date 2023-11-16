use crate::Array1;

impl<T> Array1<T>
where
    T: PartialOrd
{
    /// get the maximum value of Array1
    #[allow(unused)]
    pub fn max(&self) -> T {
        let size = self.size();
        let mut max_size = self.get(0);
        for i in 1..size {
            let value = self.get(i);
            if value > max_size {
                max_size = value;
            }
        }
        max_size
    }

    /// get the index of the maximum value of Array1
    #[allow(unused)]
    pub fn argmax(&self) -> usize {
        let size = self.size();
        let mut max_size = self.get(0);
        let mut max_index = 0;
        
        for i in 1..size {
            let value = self.get(i);
            if value > max_size {
                max_size = value;
                max_index = i;
            }
        }

        max_index
    }

}
