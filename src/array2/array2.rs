use std::alloc::Layout;

#[derive(Clone, Copy)]
pub struct Array2<T>{
    pub(crate) array: *mut T,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) layout: Layout
}
