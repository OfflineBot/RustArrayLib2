use std::ops::AddAssign;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

use crate::Array1;

impl<T> Array1<T> 
where
    T: AddAssign + Clone + PartialOrd + SampleUniform + Copy
{
    ///
    /// replaces whole Array1 with random values
    pub fn random_uniform(self, min: T, max: T) {
        let size = self.size();

        if min > max {
            panic!("minimum value is bigger than maximum value for random uniform");
        }

        let mut rng = thread_rng();
        for i in 0..size {
            let random_val = rng.gen_range(min.clone()..=max.clone());
            self.set_index(i, random_val); 
        }
    }

    pub fn random_uniform_seed(self, min: T, max: T, seed: u64) {
        let size = self.size();

        if min > max {
            panic!("minimum value is bigger than maximum value for random uniform");
        }

        let mut rng = StdRng::seed_from_u64(seed);
        for i in 0..size {
            let random_val = rng.gen_range(min.clone()..=max.clone());
            self.set_index(i, random_val);
        }
    }
}