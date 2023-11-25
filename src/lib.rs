mod traits;

mod macros;
pub use macros::array;

mod array1;
pub use array1::array1::Array1;

mod array2;
pub use array2::array2::Array2;


#[cfg(test)]
mod tester {
    use crate::*;


    #[test]
    fn x_y_norm() {
        let x: Vec<Vec<f32>> = vec![
            vec![1.0, 1.0],
            vec![0.0, 0.0],
        ];
        let y: Vec<Vec<f32>> = vec![
            vec![1.0],
            vec![0.0]
        ];
        let input: Array2<f32> = Array2::from_vec(x);
        let output: Array2<f32> = Array2::from_vec(y);

        let x_mean: Array1<f32> = input.mean();
        let x_std: Array1<f32> = input.std();
        let x_norm: Array2<f32> = (input - &x_mean) / &x_std;
        println!("x_norm: {:?}", x_norm);

        let y_mean: Array1<f32> = output.mean();
        let y_std: Array1<f32> = output.std();
        let y_norm: Array2<f32> = (output - &y_mean) / &y_std;
        println!("y_norm: {:?}", y_norm);

    }
}