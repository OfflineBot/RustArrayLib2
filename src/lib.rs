mod traits;

mod array1;
pub use array1::array1::Array1;

mod array2;
pub use array2::array2::Array2;


// testing
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
        let x: Array1<f32> = Array1::new(10);
        let min: f32 = -1.0;
        let max: f32 = 1.0;
        x.random_uniform(min, max);
        println!("{:?}", x);
    }

    #[test]
    fn test_array2() {
        let vec: Vec<Vec<f32>> = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
        ];
        let vec2: Vec<Vec<f32>> = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let arr1: Array2<f32> = Array2::new_from_vec(vec);
        let arr2: Array2<f32> = Array2::new_from_vec(vec2);
        
        println!("arr1: {:?}", arr1);
        println!("arr2: {:?}", arr2);

        let my_dot: Array2<f32> = arr1.dot(&arr2);
        println!("dot: {:?}", my_dot);
    }

    #[test]
    fn test_std2() {
        let vec: Vec<Vec<f32>> = vec![
            vec![1.0, 1.0],
            vec![0.0, 0.0],
        ];
        let arr: Array2<f32> = Array2::new_from_vec(vec);
        println!("{:?}", arr.std());
    }
}
