pub trait Exp {
    fn exp(&self) -> Self;
}

impl Exp for f32 {
    #[allow(unconditional_recursion)]
    fn exp(&self) -> f32 {
        f32::exp(*self) 
    }
}

impl Exp for f64 {
    #[allow(unconditional_recursion)]
    fn exp(&self) -> f64 {
        f64::exp(*self)
    }
}