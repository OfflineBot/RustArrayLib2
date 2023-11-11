use std::alloc::Layout;

#[derive(Clone, Copy)]
pub struct Array1<T> {
    pub array: *mut T,
    pub size: usize,
    pub layout: Layout,
}
