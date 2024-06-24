use std::{alloc, ptr};
use std::alloc::{Layout};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) struct SmartPointer<T> {
    ptr: ptr::NonNull<T>
}

impl<T> SmartPointer<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc::alloc(layout) as *mut T;
            if ptr.is_null() == true {
                alloc::handle_alloc_error(layout);
            }
            ptr::write(ptr, value);
            Self {
                ptr: ptr::NonNull::new_unchecked(ptr)
            }
        }
    }

    pub fn get_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: Clone> Clone for SmartPointer<T> {
    fn clone(&self) -> Self {
        SmartPointer::new(self.deref().clone())
    }
}

impl<T: Display> Display for SmartPointer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", *self)?;
        Ok(())
    }
}

impl<T> Deref for SmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for SmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod smart_pointer {
    use crate::structs::smart_ptr::SmartPointer;

    #[test]
    fn test_new() {
        let s_ptr = SmartPointer::new(3);
        assert_eq!(*s_ptr, 3, "Smart pointer immutable derefencing failed or diferent data is stored!");
    }

    #[test]
    fn test_deref_mut() {
        let mut s_ptr_mut = SmartPointer::new(10);
        *s_ptr_mut = 20;
        assert_eq!(*s_ptr_mut, 20, "Smart pointer mutable derefencing failed!")
    }

    // TODO: Figure out how to verify the custom memory drop logic is cleaning all memory.
    #[test]
    fn test_drop() {
        let test_data = String::from("hello");

        let s_ptr = SmartPointer::new(test_data.clone());
        assert_eq!(*s_ptr, "hello", "Smart pointer is not pointing to the correct data");
        drop(s_ptr);
    }
}
