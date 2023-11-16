use std::ops::AddAssign;
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

use crate::Array1;

use crate::traits::set_val::SetVal;

impl<T> Array1<T> 
where
    T: AddAssign + Clone + SetVal<T> + PartialOrd + SampleUniform + Copy
{
    pub fn random_uniform(self, min: T, max: T) {
        let size = self.size();

        let mut rng = thread_rng();
        for i in 0..size {
            let random_val = rng.gen_range(T::set_val(min)..=T::set_val(max));
            self.set_index(i, random_val); 
        }
    }
}