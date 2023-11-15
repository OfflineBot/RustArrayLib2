use std::ops::{Add, Sub, Mul, Div};
use std::alloc::{Layout, alloc};
use crate::Array1;

impl<T> Add<&Array1<T>> for Array1<T>
where
    T: Add<Output = T> + Clone
{
    type Output = Array1<T>;

    fn add(self, rhs: &Array1<T>) -> Array1<T> {
        let size = self.size;        
        let size2 = rhs.size;

        if size != size2 {
            panic!("sizes dont match for operation");
        }

        let lhs_array = self.array;
        let rhs_array = rhs.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);
                let ptr2 = rhs_array.offset(i as isize);
                let val2 = std::ptr::read(ptr2);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 + val2;
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

impl<T> Sub<&Array1<T>> for Array1<T>
where
    T: Sub<Output = T>
{
    type Output = Array1<T>;
    fn sub(self, rhs: &Array1<T>) -> Array1<T> {
        let size = self.size;        
        let size2 = rhs.size;

        if size != size2 {
            panic!("sizes dont match for operation");
        }

        let lhs_array = self.array;
        let rhs_array = rhs.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);
                let ptr2 = rhs_array.offset(i as isize);
                let val2 = std::ptr::read(ptr2);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 - val2;
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

impl<T> Mul<&Array1<T>> for Array1<T>
where
    T: Mul<Output = T>
{
    type Output = Array1<T>;
    fn mul(self, rhs: &Array1<T>) -> Array1<T> {
        let size = self.size;        
        let size2 = rhs.size;

        if size != size2 {
            panic!("sizes dont match for operation");
        }

        let lhs_array = self.array;
        let rhs_array = rhs.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);
                let ptr2 = rhs_array.offset(i as isize);
                let val2 = std::ptr::read(ptr2);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 * val2;
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

impl<T> Div<&Array1<T>> for Array1<T>
where
    T: Div<Output = T>
{
    type Output = Array1<T>;
    fn div(self, rhs: &Array1<T>) -> Array1<T> {
        let size = self.size;        
        let size2 = rhs.size;

        if size != size2 {
            panic!("sizes dont match for operation");
        }

        let lhs_array = self.array;
        let rhs_array = rhs.array;
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_array = unsafe { alloc(new_layout) as *mut T };

        for i in 0..size {
            unsafe {
                let ptr1 = lhs_array.offset(i as isize);
                let val1 = std::ptr::read(ptr1);
                let ptr2 = rhs_array.offset(i as isize);
                let val2 = std::ptr::read(ptr2);

                let out_ptr = new_array.offset(i as isize);
                let out_val = val1 / val2;
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