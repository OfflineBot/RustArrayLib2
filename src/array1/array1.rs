use std::alloc::Layout;

#[allow(unused)]
pub struct Array1<T> {
    pub array: *mut T,
    pub size: usize,
    pub layout: Layout,
}
