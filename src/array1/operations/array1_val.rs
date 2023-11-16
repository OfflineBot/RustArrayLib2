use std::ops::{Add, Sub, Mul, Div};
use std::alloc::{Layout, alloc};
use crate::Array1;

// reference
impl<T> Add<&T> for Array1<T>
where
    T: Add<Output = T> + Clone
{
    type Output = Array1<T>;

    fn add(self, rhs: &T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 + rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Sub<&T> for Array1<T>
where
    T: Sub<Output = T> + Clone
{
    type Output = Array1<T>;
    fn sub(self, rhs: &T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 - rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Mul<&T> for Array1<T>
where
    T: Mul<Output = T> + Clone
{
    type Output = Array1<T>;
    fn mul(self, rhs: &T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 * rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Div<&T> for Array1<T>
where
    T: Div<Output = T> + Clone
{
    type Output = Array1<T>;
    fn div(self, rhs: &T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 / rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

// no reference
impl<T> Add<T> for Array1<T>
where
    T: Add<Output = T> + Clone
{
    type Output = Array1<T>;

    fn add(self, rhs: T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 + rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Sub<T> for Array1<T>
where
    T: Sub<Output = T> + Clone
{
    type Output = Array1<T>;
    fn sub(self, rhs: T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 - rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Mul<T> for Array1<T>
where
    T: Mul<Output = T> + Clone
{
    type Output = Array1<T>;
    fn mul(self, rhs: T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 * rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}

impl<T> Div<T> for Array1<T>
where
    T: Div<Output = T> + Clone
{
    type Output = Array1<T>;
    fn div(self, rhs: T) -> Array1<T> {
        let size = self.size;        

        let lhs_array = self.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 / rhs.clone();

                std::ptr::write(out_ptr, out_val);
            };
        }
        Array1 {
            array: new_array,
            size,
            layout: new_layout
        }
    }
}