
#[macro_export]
macro_rules! array {
    ($([$($expr:expr), + $(,)?]), + $(,)?) => {{
        let mut vec = Vec::new();
        $(
            let mut slice = Vec::new();
            $(
                slice.push($expr);
            ) +
            vec.push(slice);
        ) +
        Array2::from_vec(vec)
    }};
    ($($expr:expr), + $(,)?) => {{
        let mut vec = Vec::new();
        $(
            vec.push($expr);
        ) +
        Array1::from_vec(vec)
    }};

}