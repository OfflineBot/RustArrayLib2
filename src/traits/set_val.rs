
pub trait SetVal<T> {
    fn set_val(value: T) -> T;
}

impl SetVal<f32> for f32 {
    fn set_val(value: f32) -> f32 {
        value as f32 
    }
}

impl SetVal<f64> for f64 {
    fn set_val(value: f64) -> f64 {
        value as f64
    }
}