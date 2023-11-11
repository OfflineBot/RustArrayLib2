
use std::ops::AddAssign;
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

use crate::Array2;

impl<T> Array2<T> 
where
    T: PartialOrd + AddAssign + SampleUniform + Clone
{
    pub fn random_uniform(&self, min: T, max: T) {
        
        let mut rng = thread_rng();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let random_val = rng.gen_range(min.clone()..=max.clone());
                self.set(i, j, random_val);
            }
        }
    }
}