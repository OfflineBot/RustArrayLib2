use std::alloc::Layout;

#[derive(Clone)]
pub struct Array2<T>{
    pub array: *mut T,
    pub rows: usize,
    pub cols: usize,
    pub layout: Layout
}
