use rarray::*;


fn dl() {
    let iter = 1000;
    let lr = 0.01;
    let hidden_layer = 4;

    let e_minus = 1e-4;

    let input: Array2<f32> = array![
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];
    let output: Array2<f32> = array![
        [1.0],
        [0.0],
        [0.5],
    ];
    let new_input: Array2<f32> = array![
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];

    let input_layer = input.size()[1];
    let output_layer = output.size()[1];

    let x_mean: Array1<f32> = input.mean();
    let x_std: Array1<f32> = input.std();
    x_std.replace_zero(e_minus);

    let y_mean: Array1<f32> = output.mean();
    let y_std: Array1<f32> = output.std();
    y_std.replace_zero(e_minus);

    let x_norm: Array2<f32> = (input - &x_mean) / &x_std;
    let y_norm: Array2<f32> = (output - &y_mean) / &y_std;

    let (min, max) = (-1.0, 1.0);
    let seed = 32;

    let mut w1: Array2<f32> = Array2::new(input_layer, hidden_layer); 
    let mut b1: Array1<f32> = Array1::new(hidden_layer);
    let mut w2: Array2<f32> = Array2::new(hidden_layer, output_layer);
    let mut b2: Array1<f32> = Array1::new(output_layer);
    w1.random_uniform_seed(min, max, seed);
    b1.random_uniform_seed(min, max, seed);
    w2.random_uniform_seed(min, max, seed);
    b2.random_uniform_seed(min, max, seed);

    let start_w1 = w1.clone();

    for i in 0..=iter {

        let z1 = x_norm.dot(&w1) + &b1;
        let a1 = z1.leaky_relu(e_minus);
        let z2 = a1.dot(&w2) + &b2;
        
        if i % 1_000 == 0 {
            println!("i: {}", i);
        }

        let delta2 = z2 - y_norm;
        let delta1 = delta2.dot(&w2.t()) * z1.deriv_relu();

        w1 = w1 - &(x_norm.t().dot(&delta1) * lr);
        b1 = b1 - &(delta1.sum() * lr);
        w2 = w2 - &(a1.t().dot(&delta2) * lr);
        b2 = b2 - &(delta2.sum() * lr);

    }

    let end_w1 = w1.clone();
    println!("\n\nerror w1: {:?}", start_w1 - end_w1);
    let new_norm = (new_input - &x_mean) / &x_std;
    let z1 = new_norm.dot(&w1) + &b1;
    let a1 = z1.leaky_relu(e_minus);
    let z2 = a1.dot(&w2) + &b2;
    let output = z2 * y_std * y_mean;
    println!("output: {:?}", output);

}


fn main() {
    let array1 = array![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];
    let array2 = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ];
    let array3 = array![
        [1.0, 2.0, 3.0]
    ];
    let mut out = array1.dot(&array2);
    println!("hello");
    out = out + &array3;
    println!("out: {:?}", out);
}