pub trait SetOne {
    fn set_one() -> Self;
}

impl SetOne for f32 {
    fn set_one() -> f32 {
        1.0
    } 
}

impl SetOne for f64 {
    fn set_one() -> f64 {
        1.0
    }
}

impl SetOne for usize {
    fn set_one() -> usize {
        1
    }
}

impl SetOne for isize {
    fn set_one() -> isize {
        1
    }
}