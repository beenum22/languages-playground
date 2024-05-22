#![allow(dead_code)]
// use crate::traits::GetPointer;

use std::alloc::Layout;
use std::{alloc, fmt, ptr};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut, Add, Div};
use std::default::Default;
use num::Zero;

pub struct MyArray<T> {
    ptr: *mut T,
    size: usize,
    pub(crate) length: usize,
}

impl<T: Default + PartialEq > MyArray<T> {
    pub(crate) fn new(size: usize) -> Self {
        let layout = Layout::array::<T>(size).expect("Layout creation failed");
        let ptr = unsafe { alloc::alloc(layout) as * mut T };

        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        MyArray {
            ptr,
            size,
            length: 0
        }
    }

    pub(crate) fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub(crate) fn get_ref(&self) -> &T {
        unsafe {
            &*self.ptr
        }
    }

    pub(crate) fn push(&mut self, value: T) {
        if self.length == self.size {
            panic!("Array is already full!");
        }

        unsafe {
            ptr::write(self.ptr.add(self.length), value);
        }
        self.length += 1;
    }

    pub(crate) fn insert(&mut self, index: usize, value: T) -> () {
        if self.length == self.size {
            panic!("Array is already full!");
        } else if index > self.length {
            panic!("Index is greater than the length of an Array!")
        }
        let mut i:usize = self.length;
        while i > index {
            unsafe {
                ptr::write(self.ptr.add(i), ptr::read(self.ptr.add(i - 1)));
            }
            i -= 1;
        }
        unsafe {
            ptr::write(self.ptr.add(i), value);
            self.length += 1;
        }
    }

    pub(crate) fn delete(&mut self, index: usize) -> () {
        if index >= self.length {
            panic!("Index is greater than Array length!");
        }
        unsafe {
            ptr::write(self.ptr.add(index), T::default());
            // ptr::null(self.ptr.add(index));
        }
        let mut i: usize = index;
        while i < self.length - 1 {
            unsafe {
                ptr::write(self.ptr.add(i), ptr::read(self.ptr.add(i + 1)));
            }
            i += 1;
        }
        unsafe {
            ptr::write(self.ptr.add(i), T::default());
        }
        self.length -= 0;
    }

    pub(crate) fn linear_search(self, value: T) -> Option<usize> {
        let mut i: usize = 0;
        while i < self.length {
            unsafe {
                if ptr::read(self.ptr.add(i)) == value {
                    return Some(i)
                }
            }
            i += 1;
        }
        return None
    }

    unsafe fn swap_keys(&self, index_1: usize, index_2: usize) -> () {
        let value_1 = ptr::read(self.ptr.add(index_1));
        let value_2 = ptr::read(self.ptr.add(index_2));
        ptr::write(self.ptr.add(index_1), value_2);
        ptr::write(self.ptr.add(index_2), value_1);
    }

    pub(crate) fn transposition_search(&self, value: T) -> Option<usize> {
        let mut i: usize = 0;
        while i < self.length {
            unsafe {
                if ptr::read(self.ptr.add(i)) == value {
                    self.swap_keys(i-1, i);
                    return Some(i)
                }
            }
            i += 1;
        }
        return None
    }

    pub(crate) fn move_to_head_search(&self, value: T) -> Option<usize> {
        let mut i: usize = 0;
        while i < self.length {
            unsafe {
                // if ptr::read(self.ptr.add(i)) == value {
                if &*self.ptr.add(i) == &value {
                    self.swap_keys(0, i);
                }
            }
            i += 1;
        }
        return None
    }

    pub(crate) fn binary_search(&self, value: T) -> Option<usize>
        where T: PartialOrd
    {
        let mut low: usize = 0;
        let mut high: usize = self.length - 1;
        while low <= high {
            let mid: usize = (low + high)/2;
            let mid_value: &T = unsafe {&*self.ptr.add(mid)};
            // let mid_value: T = unsafe {ptr::read(self.ptr.add(mid))};
            if &value == mid_value {
                return Some(mid)
            } else if &value < mid_value {
                high = mid - 1;
            } else if &value > mid_value {
                low = mid + 1
            }
        }
        return None
    }

    pub(crate) fn recursive_binary_search(&self, low: usize, high: usize, value: T) -> Option<usize>
        where T: PartialOrd
    {
        if low <= high {
            let mid: usize = (low + high)/2;
            // let mid_value: T = unsafe {ptr::read(self.ptr.add(mid))};
            let mid_value: &T = unsafe {&*self.ptr.add(mid)};
            if &value == mid_value {
                return Some(mid)
            } else if &value < mid_value {
                return self.recursive_binary_search(low, mid - 1, value)
            } else if &value > mid_value {
                return self.recursive_binary_search(mid + 1, high, value)
            }
        }
        return None
    }

    // Time Complexity is constant
    pub(crate) fn get(&self, index: usize) -> &T {
        if index >= self.length {
            // return None;
            panic!("Invalid index provided!");
        }

        unsafe {
            // ptr::read(self.ptr.add(index) as *const &T)
            &*self.ptr.add(index)
        }
    }

    // Time Complexity is constant
    pub(crate) fn get_optional(&self, index: usize) -> Option<&T> {
        return if index < self.length {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }

    pub(crate) fn set(&mut self, index: usize, value: T) -> () {
        if index >= self.length {
            panic!("Invalid index provided!");
        }

        unsafe {
            ptr::write(self.ptr.add(index), value)
        }
    }

    // Time complexity is O(n)
    pub(crate) fn max(&self) -> &T
        where T: PartialOrd
    {
        let mut max: &T = self.get(0);
        for i in 1..self.length {
            let target = self.get(i);
            if target > max {
                max = target;
            }
        }
        return max
    }

    // Time complexity is O(n)
    pub(crate) fn min(&self) -> &T
        where T: PartialOrd
    {
        let mut min: &T = self.get(0);
        for i in 1..self.length {
            let target = self.get(i);
            if target < min {
                min = target;
            }
        }
        return min
    }

    // Time complexity is O(n)
    pub(crate) fn sum(&self) -> T
        where T: Add<Output = T> + Zero + Copy,
    {
        let mut sum: T = T::zero();
        for i in 0..self.length {
            sum = sum + *self.get(i)
        }
        return sum
    }

    // Time complexity is O(n)
    pub(crate) fn recursive_sum(&self, n: usize) -> T
        where T: Add<Output = T> + Zero + Copy,
    {
        return if n >= 0 {
            match n.checked_sub(1) {
                Some(n_out) => self.recursive_sum(n_out) + *self.get(n),
                None => *self.get(n)
            }
        } else {
            T::zero()
        }
    }

    // Time complexity is O(n)
    pub(crate) fn avg(&self) -> T
        where T: Div<Output = T> + Copy + Zero + From<usize>,
    {
        return self.sum() / T::from(self.length)
    }

    // Time complexity is O(n)
    pub(crate) fn copy_reverse(&mut self) -> ()
        where T: Copy
    {
        let mut copy = MyArray::new(self.size);
        for i in (0..self.length).rev() {
            copy.push(*self.get(i))
        }
        for i in 0..self.length {
            self.set(i, copy[i]);
        }
    }

    // Time complexity is O(n)
    pub(crate) fn swap_reverse(&mut self) -> ()
        where T: Copy
    {
        let mut i: usize = 0;
        let mut j: usize = self.length - 1;
        while i < j {
            let temp_holder = self[i];
            self.set(i, self[self.length - 1 - i]);
            self.set(self.length - 1 - i, temp_holder);
            i += 1;
            j -= 1;
        }
    }

    // Time complexity is O(n)
    pub(crate) fn left_shift(&mut self) -> ()
        where T: Copy
    {
        let temp_val = self[0];
        for i in 1..self.length {
            self.set(i - 1,self[i]);
            if i == self.length - 1 {
                self[i] = temp_val;
            }
        }
    }

    // Time complexity is O(n)
    pub(crate) fn right_shift(&mut self) -> ()
        where T: Copy
    {
        let temp_val = self[self.length - 1];
        for i in (0..(self.length-1)).rev() {
            self.set(i + 1,self[i]);
            if i == 0 {
                self[i] = temp_val;
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for MyArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            unsafe {
                write!(f, "{}", &*self.ptr.add(i))?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> Index<usize> for MyArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.size {
            unsafe {
                &*self.ptr.add(index)
            }
        } else {
            panic!("Index out of range!");
        }
    }
}

impl<T> IndexMut<usize> for MyArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.size {
            if index >= self.length {
                // self.length = index + 1;
                self.length += 1;
            }
            unsafe {
                &mut *self.ptr.add(index)
            }
        } else {
            panic!("Index out of range!");
        }
    }
}

impl<T> Drop for MyArray<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.size).expect("Layout creation failed");

        unsafe {
            for i in 0..self.length {
                ptr::drop_in_place(self.ptr.add(i));
            }
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

// pub struct VisualizeDataType<'a, T> {
//     // pub(crate) ptr: *const T,
//     pub(crate) value: &'a T
// }
//
// impl<'a, T> VisualizeDataType<'a, T> {
//     pub(crate) fn init(value: &'a T) -> Self {
//         VisualizeDataType {
//             // ptr: value as *const T,
//             value,
//         }
//     }
// }
//
// impl<'a, T: fmt::Display> fmt::Display for VisualizeDataType<'a, T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let default_ptr_len: usize = mem::size_of::<*const u16>();
//         write!(f, "┌{}┐\n", "─".repeat(default_ptr_len * 2))?;
//         write!(f, "│ value={}{}│\n", self.value, " ".repeat(default_ptr_len*2))?;
//         // write!(f, "│ ptr={:p} │\n", self.ptr)?;
//         write!(f, "│ ptr={:p} │\n", self.get_ptr())?;
//         write!(f, "└{}┘", "─".repeat(default_ptr_len * 2))?;
//         Ok(())
//     }
// }
