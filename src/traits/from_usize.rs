pub trait FromUsize<T> {
    fn from_usize(value: usize) -> T;
}

impl FromUsize<f32> for f32 {
    fn from_usize(value: usize) -> f32 {
        value as f32
    }
}

impl FromUsize<f64> for f64 {
    fn from_usize(value: usize) -> f64 {
        value as f64
    }
}