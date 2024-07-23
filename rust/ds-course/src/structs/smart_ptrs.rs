use std::{alloc, mem, ptr};
use std::alloc::{Layout};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};


// Imitates Box from Rust
#[derive(Debug)]
#[derive(PartialEq)]
pub struct HeapBox<T> {
    ptr: NonNull<T>
}

impl<T> HeapBox<T> {
    // TODO: Add error handling when it is unable to allocate. It would be needed in Linked List and later Stack full errors.
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

    pub fn leak(smart_ptr: HeapBox<T>) -> NonNull<T> {
        let ptr = NonNull::new(smart_ptr.ptr.as_ptr());
        mem::forget(smart_ptr);
        ptr.unwrap()
    }

    pub fn unleak(ptr: NonNull<T>) -> HeapBox<T> {
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: Clone> Clone for HeapBox<T> {
    fn clone(&self) -> Self {
        HeapBox::new(self.deref().clone())
    }
}

impl<T: Display> Display for HeapBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", *self)?;
        Ok(())
    }
}

impl<T> Deref for HeapBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for HeapBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for HeapBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[derive(Debug)]
pub struct AtomicReferenceState<T> {
    value: T,
    count: UnsafeMutable<AtomicUsize>
}

impl<T> AtomicReferenceState<T>{
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

impl<T: Clone> Clone for AtomicReferenceState<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            count: unsafe { UnsafeMutable::new(AtomicUsize::new((*self.count.get()).load(Ordering::SeqCst))) },
        }
    }
}

// Imitates Arc from Rust
#[derive(PartialEq)]
pub struct AtomicReferenceCounter<T> {
    ptr: NonNull<AtomicReferenceState<T>>
    // ptr: *mut ReferenceCounter<T>
    // ptr: &'static mut ReferenceCounter<T>
}

impl<T> AtomicReferenceCounter<T> {
    pub fn new(val: T) -> Self {
        let rc = AtomicReferenceState::new(val);
        let smart_ptr = HeapBox::new(rc);
        let leak_ptr = HeapBox::leak(smart_ptr);
        Self {
            ptr: leak_ptr
        }
    }

    pub fn as_ref(&self) -> &AtomicReferenceState<T> {
        unsafe {
            &*(self.ptr.as_ptr() as *const AtomicReferenceState<T>)
        }
    }

    pub fn as_mut(&mut self) -> &mut AtomicReferenceState<T> {
        unsafe {
            &mut *(self.ptr.as_ptr())
        }
    }

    pub fn as_ptr(&self) -> *mut AtomicReferenceState<T> {
        self.ptr.as_ptr()
    }

    pub fn count(&self) -> usize {
        self.as_ref().count()
    }

    pub fn increment(&self) {
        self.as_ref().increment()
    }

    pub fn decrement(&self) {
        self.as_ref().decrement()
    }
}

impl<T> Clone for AtomicReferenceCounter<T> {
    fn clone(&self) -> Self {
        self.increment();
        AtomicReferenceCounter {
            ptr: self.ptr
        }
    }
}

impl<T> Deref for AtomicReferenceCounter<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.as_ref().value
    }
}

impl<T: Debug> Debug for AtomicReferenceCounter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedSmartPointer")
            .field("ptr", &self.ptr)
            .field("value", &self.as_ref().value)
            .field("count", &self.count())
            .finish()
    }
}

impl<T> DerefMut for AtomicReferenceCounter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.as_mut().value
    }
}

impl<T> Drop for AtomicReferenceCounter<T> {
    fn drop(&mut self) {
        self.decrement();
        if self.count() == 0usize {
            drop(HeapBox::unleak(self.ptr))
        }
    }
}

// Imitates UnsafeCell
#[derive(Debug)]
pub struct UnsafeMutable<T> {
    value: T
}

// Note:
// Imitates Rust Cell. However, the get method here should just return the copy of the value and
// as_ptr method should return the mutable pointer instead.
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
mod heap_box {
    use std::ptr::NonNull;
    use crate::structs::smart_ptrs::HeapBox;

    #[test]
    fn test_new() {
        let s_ptr = HeapBox::new(3);
        assert_eq!(*s_ptr, 3, "Smart pointer immutable derefencing failed or diferent data is stored!");
    }

    #[test]
    fn test_leak() {
        let global_ref: NonNull<u8>;
        {
            let s_ptr: HeapBox<u8> = HeapBox::new(3);
            let s_ptr_leaked = HeapBox::leak(s_ptr);
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
        let s_ptr: HeapBox<u8> = HeapBox::new(10);
        unsafe { assert_eq!(*s_ptr.as_ptr(), 10u8, "Failed to deref return pointer!"); }
    }

    #[test]
    fn test_clone() {
        let s_ptr = HeapBox::new(10);
        let clone_s_ptr = s_ptr.clone();
        assert_ne!(s_ptr.ptr, clone_s_ptr.ptr, "Cloned pointer is the same as original one!");
        assert_eq!(*s_ptr, *clone_s_ptr, "Cloned data is different!")
    }

    #[test]
    fn test_deref_mut() {
        let mut s_ptr_mut = HeapBox::new(10);
        *s_ptr_mut = 20;
        assert_eq!(*s_ptr_mut, 20, "Smart pointer mutable derefencing failed!")
    }

    // TODO: Figure out how to verify the custom memory drop logic is cleaning all memory.
    #[test]
    fn test_drop() {
        let test_data = String::from("hello");

        let s_ptr = HeapBox::new(test_data.clone());
        assert_eq!(*s_ptr, "hello", "Smart pointer is not pointing to the correct data");
        drop(s_ptr);
    }
}

mod atomic_reference_counter {
    use super::*;

    #[test]
    fn test_new() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        assert_eq!(*shared, 1, "Invalid data found in the Shared pointer!");
        assert_eq!(shared.count(), 1usize, "Invalid reference count found in the Shared pointer!");
    }

    #[test]
    fn test_as_ref() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        assert_eq!(shared.as_ref().value, 1u8, "Invalid data found in the Reference Counter reference!");
    }

    #[test]
    fn test_as_mut() {
        let mut shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        assert_eq!(shared.as_mut().value, 1u8, "Invalid data found in the Reference Counter mutable reference!");
    }

    #[test]
    fn test_count() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        assert_eq!(shared.count(), 1usize, "Invalid count found in the Reference Counter!");
    }

    #[test]
    fn test_increment() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        shared.increment();
        assert_eq!(shared.count(), 2usize, "Invalid increment in the Reference Counter!");
    }

    #[test]
    fn test_decrement() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        shared.decrement();
        assert_eq!(shared.count(), 0usize, "Invalid decrement in the Reference Counter!");
    }

    #[test]
    fn test_clone() {
        let shared: AtomicReferenceCounter<u8> = AtomicReferenceCounter::new(1);
        {
            assert_eq!(shared.clone().count(), 2, "Invalid reference count found in the Shared pointer!");
        }
        assert_eq!(shared.count(), 1, "Invalid reference count found in the Shared pointer!");
    }
}

mod atomic_reference_state {
    use std::sync::atomic::{Ordering};
    use super::*;

    #[test]
    fn test_new() {
        let rc: AtomicReferenceState<u8> = AtomicReferenceState::new(1);
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 1, "Invalid initial reference count!");
        }
    }

    #[test]
    fn test_increment() {
        let rc: AtomicReferenceState<u8> = AtomicReferenceState::new(1);
        rc.increment();
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 2, "Invalid incremented reference count!");
        }
    }

    #[test]
    fn test_decrement() {
        let rc: AtomicReferenceState<u8> = AtomicReferenceState::new(1);
        rc.decrement();
        unsafe {
            assert_eq!((*rc.count.get()).load(Ordering::SeqCst), 0, "Invalid decremented reference count!");
        }
    }

    #[test]
    fn test_count() {
        let rc: AtomicReferenceState<u8> = AtomicReferenceState::new(1);
        assert_eq!(rc.count(), 1, "Invalid reference count!");
    }
}

mod unsafe_mutable {
    use super::*;

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
