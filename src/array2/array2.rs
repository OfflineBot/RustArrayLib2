use std::alloc::Layout;

#[derive(Clone, Copy)]
pub struct Array2<T>{
    pub array: *mut T,
    pub rows: usize,
    pub cols: usize,
    pub layout: Layout
}