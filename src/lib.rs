mod array1;
pub use array1::array1::Array1;

mod array2;
pub use array2::array2::Array2;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let vec: Vec<f32> = vec![3.3, 5.3, 7.6];
        let mut x: Array1<f32> = Array1::new_from_vec(vec);
        x.add(4.3);
        println!("{:?}", x);
    }

    #[test]
    fn random_test() {
        let mut x: Array1<f32> = Array1::new(10);
        let min: f32 = -1.0;
        let max: f32 = 1.0;
        x.random_uniform(min, max);
        println!("{:?}", x);
    }
}