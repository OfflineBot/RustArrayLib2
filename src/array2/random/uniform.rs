
use std::ops::AddAssign;
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform, rngs::StdRng, SeedableRng};

use crate::Array2;

impl<T> Array2<T> 
where
    T: PartialOrd + AddAssign + SampleUniform + Clone
{
    pub fn random_uniform(&self, min: T, max: T) {
        
        if min > max {
            panic!("minimum value is bigger than maximum value for random uniform");
        }

        let mut rng = thread_rng();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let random_val = rng.gen_range(min.clone()..=max.clone());
                self.set(i, j, random_val);
            }
        }
    }

    pub fn random_uniform_seed(self, min: T, max: T, seed: u64) {
        
        if min > max {
            panic!("minimum value is bigger than maximum value for random uniform");
        }

        let mut rng = StdRng::seed_from_u64(seed);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let random_val = rng.gen_range(min.clone()..=max.clone());
                self.set(i, j, random_val);
            }
        }
    }
}