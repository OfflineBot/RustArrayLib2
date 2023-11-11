use std::ops::AddAssign;
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

use crate::Array1;

impl<T> Array1<T> 
where
    T: PartialOrd + AddAssign + SampleUniform + Clone
{
    pub fn random_uniform(&self, min: T, max: T) {
        let size = self.size;

        let mut rng = thread_rng();
        for i in 0..size {
            unsafe {
                let value = self.array.offset(i as isize);
                let random_val = rng.gen_range(min.clone()..=max.clone());
                std::ptr::write(value, random_val)
            }
        }

    }
}