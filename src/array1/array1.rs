use std::alloc::Layout;

#[derive(Clone, Copy)]
pub struct Array1<T> {
    pub(crate) array: *mut T,
    pub(crate) size: usize,
    pub(crate) layout: Layout,
}
