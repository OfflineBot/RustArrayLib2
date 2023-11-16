mod traits;

mod array1;
pub use array1::array1::Array1;

mod array2;
pub use array2::array2::Array2;


// testing
#[cfg(test)]
mod tests {
    use crate::Array1;

    #[test]
    fn my_test() {
        let x: Array1<f32> = Array1::new(3);
    }
}
