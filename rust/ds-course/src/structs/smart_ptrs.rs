use std::{alloc, mem, ptr};
use std::alloc::{Layout};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};


// Imitates Box from Rust
#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) struct SmartPointer<T> {
    ptr: NonNull<T>
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
                ptr: NonNull::new_unchecked(ptr)
            }
        }
    }

    pub fn leak(mut smart_ptr: SmartPointer<T>) -> NonNull<T> {
        unsafe {
            let ptr = NonNull::new(smart_ptr.ptr.as_ptr());
            mem::forget(smart_ptr);
            ptr.unwrap()
        }
    }

    pub fn unleak(ptr: NonNull<T>) -> SmartPointer<T> {
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut T {
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

pub struct ReferenceCounter<T> {
    value: T,
    count: UnsafeMutable<AtomicUsize>
}

impl<T> ReferenceCounter<T>{
    fn new(val: T) -> Self {
        Self {
            value: val,
            count: UnsafeMutable::new(AtomicUsize::new(1))
        }
    }

    fn increment(&self) {
        unsafe {
            (*self.count.get()).fetch_add(1, Ordering::SeqCst);
        }
    }

    fn decrement(&self) {
        unsafe {
            (*self.count.get()).fetch_sub(1, Ordering::SeqCst);
        }
    }

    fn count(&self) -> usize {
        unsafe {
            (*self.count.get()).load(Ordering::SeqCst)
        }
    }

}

impl<T: Clone> Clone for ReferenceCounter<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            count: unsafe { UnsafeMutable::new(AtomicUsize::new((*self.count.get()).load(Ordering::SeqCst))) },
        }
    }
}

// Imitates Rc from Rust
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SharedSmartPointer<T> {
    ptr: NonNull<ReferenceCounter<T>>
    // ptr: *mut ReferenceCounter<T>
    // ptr: &'static mut ReferenceCounter<T>
}

impl<T> SharedSmartPointer<T> {
    pub fn new(val: T) -> Self {
        let rc = ReferenceCounter::new(val);
        let smart_ptr = SmartPointer::new(rc);
        let leak_ptr = SmartPointer::leak(smart_ptr);
        Self {
            ptr: leak_ptr
        }
    }

    pub fn as_ref(&self) -> &ReferenceCounter<T> {
        unsafe {
            &*(self.ptr.as_ptr() as *const ReferenceCounter<T>)
        }
    }

    pub fn as_mut(&mut self) -> &mut ReferenceCounter<T> {
        unsafe {
            &mut *(self.ptr.as_ptr())
        }
    }

    pub fn as_ptr(&self) -> *mut ReferenceCounter<T> {
        self.ptr.as_ptr()
    }

    pub fn count(&self) -> usize {
        unsafe {
            self.as_ref().count()
        }
    }

    pub fn increment(&self) {
        unsafe {
            self.as_ref().increment()
        }
    }

    pub fn decrement(&self) {
        unsafe {
            self.as_ref().decrement()
        }
    }
}

impl<T> Clone for SharedSmartPointer<T> {
    fn clone(&self) -> Self {
        self.increment();
        SharedSmartPointer {
            ptr: self.ptr
        }
    }
}

impl<T> Deref for SharedSmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.as_ref().value
    }
}

impl<T> DerefMut for SharedSmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.as_mut().value
    }
}

impl<T> Drop for SharedSmartPointer<T> {
    fn drop(&mut self) {
        self.decrement();
        if self.count() == 0usize {
            drop(SmartPointer::unleak(self.ptr))
        }
    }
}

// Imitates UnsafeCell
pub struct UnsafeMutable<T> {
    value: T
}

impl<T> UnsafeMutable<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: val
        }
    }

    pub fn get(&self) -> *mut T {
        // Weird unsafe type casting to bypass Rust borrowing rules
        self as *const UnsafeMutable<T> as *mut UnsafeMutable<T> as *mut T
    }
}

#[cfg(test)]
mod smart_pointer {
    use std::ptr::NonNull;
    use crate::structs::smart_ptrs::SmartPointer;

    #[test]
    fn test_new() {
        let s_ptr = SmartPointer::new(3);
        assert_eq!(*s_ptr, 3, "Smart pointer immutable derefencing failed or diferent data is stored!");
    }

    #[test]
    fn test_leak() {
        let mut global_ref: NonNull<u8>;
        {
            let s_ptr: SmartPointer<u8> = SmartPointer::new(3);
            let s_ptr_leaked = SmartPointer::leak(s_ptr);
            unsafe {
                *s_ptr_leaked.as_ptr() = 4;
                assert_eq!(*s_ptr_leaked.as_ptr(), 4u8, "Leaked Smart Pointer is not mutable and also not trackable!");
            };
            global_ref = s_ptr_leaked;
        }
        unsafe { assert_eq!(*global_ref.as_ptr(), 4u8, "Leaked Smart Pointer is dropped!"); }
    }

    #[test]
    fn test_as_ptr() {
        let s_ptr: SmartPointer<u8> = SmartPointer::new(10);
        unsafe { assert_eq!(*s_ptr.as_ptr(), 10u8, "Failed to deref return pointer!"); }
    }

    #[test]
    fn test_clone() {
        let s_ptr = SmartPointer::new(10);
        let clone_s_ptr = s_ptr.clone();
        assert_ne!(s_ptr.ptr, clone_s_ptr.ptr, "Cloned pointer is the same as original one!");
        assert_eq!(*s_ptr, *clone_s_ptr, "Cloned data is different!")
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

mod shared_smart_pointer {
    use crate::structs::smart_ptrs::SharedSmartPointer;

    #[test]
    fn test_new() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        assert_eq!(*shared, 1, "Invalid data found in the Shared pointer!");
        assert_eq!(shared.count(), 1usize, "Invalid reference count found in the Shared pointer!");
    }

    #[test]
    fn test_as_ref() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        assert_eq!(shared.as_ref().value, 1u8, "Invalid data found in the Reference Counter reference!");
    }

    #[test]
    fn test_as_mut() {
        let mut shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        assert_eq!(shared.as_mut().value, 1u8, "Invalid data found in the Reference Counter mutable reference!");
    }

    #[test]
    fn test_count() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        assert_eq!(shared.count(), 1usize, "Invalid count found in the Reference Counter!");
    }

    #[test]
    fn test_increment() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        shared.increment();
        assert_eq!(shared.count(), 2usize, "Invalid increment in the Reference Counter!");
    }

    #[test]
    fn test_decrement() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        shared.decrement();
        assert_eq!(shared.count(), 0usize, "Invalid decrement in the Reference Counter!");
    }

    #[test]
    fn test_clone() {
        let shared: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
        {
            let clone = shared.clone();
            assert_eq!(shared.count(), 2, "Invalid reference count found in the Shared pointer!");
        }
        assert_eq!(shared.count(), 1, "Invalid reference count found in the Shared pointer!");
    }
}

mod reference_counter {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use crate::structs::smart_ptrs::ReferenceCounter;

    #[test]
    fn test_new() {
        let rc: ReferenceCounter<u8> = ReferenceCounter::new(1);
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 1, "Invalid initial reference count!");
        }
    }

    #[test]
    fn test_increment() {
        let mut rc: ReferenceCounter<u8> = ReferenceCounter::new(1);
        rc.increment();
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 2, "Invalid incremented reference count!");
        }
    }

    #[test]
    fn test_decrement() {
        let mut rc: ReferenceCounter<u8> = ReferenceCounter::new(1);
        rc.decrement();
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 0, "Invalid decremented reference count!");
        }
    }

    #[test]
    fn test_count() {
        let mut rc: ReferenceCounter<u8> = ReferenceCounter::new(1);
        assert_eq!(rc.count(), 1, "Invalid reference count!");
    }
}

mod unsafe_mutable {
    use crate::structs::smart_ptrs::UnsafeMutable;

    #[test]
    fn test_new() {
        let um: UnsafeMutable<u8> = UnsafeMutable::new(1);
        assert_eq!(um.value, 1, "Invalid value store in the instance!")
    }

    #[test]
    fn test_get() {
        let val: u8 = 1;
        let um: UnsafeMutable<u8> = UnsafeMutable::new(val);
        unsafe {
            *um.get() = 2
        }
        assert_eq!(um.value, 2,"Failed to mutate the value in immutable wrapper!")
    }
}
