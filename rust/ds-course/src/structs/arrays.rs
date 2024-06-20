#![allow(dead_code)]
// use crate::traits::GetPointer;

use std::alloc::Layout;
use std::{alloc, fmt, ptr, slice};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Add, Div};
use std::default::Default;
use num::{FromPrimitive, Zero};

pub struct ArrayIterator<'a, T> {
    array: &'a HeapArray<T>,
    index: usize,
}

impl<'a, T> Iterator for ArrayIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.get_len() {
            let value = unsafe { &*self.array.ptr.add(self.index) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub struct HeapArray<T> {
    ptr: *mut T,
    size: usize,
    length: usize,
}

impl<T> HeapArray<T> {
    fn init_mem(size: usize) -> Result<(*mut T, Layout), &'static str> {
        let layout = Layout::array::<T>(size).expect("Layout creation failed");
        let ptr = unsafe { alloc::alloc_zeroed(layout) as * mut T };
        match ptr.is_null() {
            false => Ok((ptr, layout)),
            true => Err("Memory allocation failed")
        }
    }

    pub fn iter(&self) -> ArrayIterator<T>{
        ArrayIterator {
            array: self,
            index: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            size: 0,
            length: 0,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        if size == 0 {
            return Self::new();
        }
        let (ptr, _layout) = match Self::init_mem(size) {
            Ok(ptr) => ptr,
            Err(e) => panic!("Failed to initialize the array. {}", e)
        };

        HeapArray {
            ptr,
            size,
            length: 0
        }
    }

    pub(crate) fn values(values: &[T]) -> Self {
        let size: usize = values.len();
        let (ptr, _layout) = match Self::init_mem(size) {
            Ok(ptr) => ptr,
            Err(e) => panic!("Failed to initialize the array. {}", e)
        };

        unsafe {
            ptr::copy_nonoverlapping(values.as_ptr(), ptr, size);
        }

        HeapArray {
            ptr,
            size,
            length: size
        }
    }

    pub(crate) fn resize(&mut self, size: usize) -> Result<(), &'static str> {
        if size < self.length {
            // panic!("Resize capacity is less than the Array length");
            return Err("Resize capacity is less than the Array length")
        }
        if size == self.size {
            // panic!("Resize capacity is already updated");
            return Err("Resize capacity is already updated");
        }
        let (new_ptr, _new_layout) = Self::init_mem(size).unwrap();
        let old_layout = Layout::array::<T>(self.size).expect("Layout creation failed");
        unsafe {
            for i in 0..self.length {
                ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(i), 1);
            }
            alloc::dealloc(self.ptr as *mut u8, old_layout);
            self.ptr = new_ptr;
            self.size = size;
        }
        Ok(())
    }

    // TODO: Revisit this method. It might be problematic
    pub(crate) fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr as *const u8, self.length)
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

    // TODO: This method might not be needed. Remove after verification.
    pub(crate) fn get_ref(&self) -> &T {
        unsafe {
            &*self.ptr
        }
    }

    pub(crate) fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub(crate) fn get_len(&self) -> usize {
        self.length
    }

    pub(crate) fn get_size(&self) -> usize {
        self.size
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

    pub(crate) fn delete(&mut self, index: usize) -> ()
        where T: Default
    {
        if index >= self.length {
            panic!("Index is greater than Array length!");
        }
        unsafe {
            ptr::write(self.ptr.add(index), T::default());
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
        self.length -= 1;
    }

    pub fn fill(&mut self, val: T)
        where T: Copy
    {
        let start_index = match self.length {
            0 => 0,
            _ => self.length - 1
        };
        for _i in start_index..self.size {
            self.push(val);
            // self.push(val.clone());
        }
    }

    // Time Complexity is O(n)
    pub(crate) fn sorted_difference(&mut self, other: &HeapArray<T>) -> ()
        where T: PartialOrd
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let new_ptr = Self::init_mem(self.length + other.length).unwrap().0;
        let old_layout = Layout::array::<T>(self.size).expect("Layout creation failed");

        while i < self.length && j < other.length {
            if self[i] < other[j] {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
                }
                i += 1;
                k += 1;
            } else if other[j] < self[i] {
                j += 1;
            } else if self[i] == other[j] {
                i += 1;
                j += 1;
            }
        }
        while i < self.length {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
            }
            i += 1;
            k += 1;
        }
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, old_layout);
        }
        self.size = self.length + other.length;
        self.length = k;
        self.ptr = new_ptr;
        self.resize(k).expect("Array resize failed");
    }

    // Time Complexity is O(n)
    pub(crate) fn sorted_intersection(&mut self, other: &HeapArray<T>) -> ()
        where T: PartialOrd + Display
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let new_ptr = Self::init_mem(self.length + other.length).unwrap().0;
        let old_layout = Layout::array::<T>(self.size).expect("Layout creation failed");

        while i < self.length && j < other.length {
            if self[i] < other[j] {
                i += 1;
            } else if other[j] < self[i] {
                j += 1;
            } else if self[i] == other[j] {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
                }
                i += 1;
                j += 1;
                k += 1;
            }
        }
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, old_layout);
        }
        self.size = self.length + other.length;
        self.length = k;
        self.ptr = new_ptr;
        self.resize(k).expect("Array resize failed");
    }

    // Time Complexity is O(n)
    pub(crate) fn sorted_union(&mut self, other: &HeapArray<T>) -> ()
        where T: PartialOrd + Display
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let new_ptr = Self::init_mem(self.length + other.length).unwrap().0;
        let old_layout = Layout::array::<T>(self.size).expect("Layout creation failed");

        while i < self.length && j < other.length {
            if self[i] < other[j] {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
                }
                i += 1;
                k += 1;
            } else if other[j] < self[i] {
                unsafe {
                    ptr::copy_nonoverlapping(other.ptr.add(j), new_ptr.add(k), 1);
                }
                j += 1;
                k += 1;
            } else {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
                }
                i += 1;
                j += 1;
                k += 1;
            }
        }
        while i < self.length {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
            }
            i += 1;
            k += 1;
        }
        while j < other.length {
            unsafe {
                ptr::copy_nonoverlapping(other.ptr.add(j), new_ptr.add(k), 1);
            }
            j += 1;
            k += 1;
        }
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, old_layout);
        }
        self.size = self.length + other.length;
        self.length = k;
        self.ptr = new_ptr;
        self.resize(self.length).expect("Array resize failed");
    }

    // Time Complexity is ??
    pub(crate) fn sorted_merge(&mut self, other: &HeapArray<T>) -> ()
        where T: PartialOrd
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let new_ptr = Self::init_mem(self.length + other.length).unwrap().0;
        let old_layout = Layout::array::<T>(self.size).expect("Layout creation failed");

        while i < self.length && j < other.length {
            if self[i] < other[j] {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
                }
                i += 1;
                k += 1;
            } else {
                unsafe {
                    ptr::copy_nonoverlapping(other.ptr.add(j), new_ptr.add(k), 1);
                }
                j += 1;
                k += 1;
            }
        }
        while i < self.length {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(k), 1);
            }
            i += 1;
            k += 1;
        }
        while j < other.length {
            unsafe {
                ptr::copy_nonoverlapping(other.ptr.add(j), new_ptr.add(k), 1);
            }
            j += 1;
            k += 1;
        }
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, old_layout);
        }
        self.size = self.length + other.length;
        self.length = self.length + other.length;
        self.ptr = new_ptr;
    }

    pub(crate) fn linear_search(&self, value: T) -> Option<usize>
        where T: PartialEq
    {
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

    pub fn swap(&self, index_1: usize, index_2: usize) -> () {
        unsafe {
            let value_1 = ptr::read(self.ptr.add(index_1));
            let value_2 = ptr::read(self.ptr.add(index_2));
            ptr::write(self.ptr.add(index_1), value_2);
            ptr::write(self.ptr.add(index_2), value_1);
        }
    }

    pub(crate) fn transposition_search(&self, value: T) -> Option<usize>
        where T: PartialEq
    {
        let mut i: usize = 0;
        while i < self.length {
            unsafe {
                if ptr::read(self.ptr.add(i)) == value {
                    self.swap(i-1, i);
                    return Some(i)
                }
            }
            i += 1;
        }
        return None
    }

    pub(crate) fn move_to_head_search(&self, value: T) -> Option<usize>
        where T: PartialEq
    {
        let mut i: usize = 0;
        while i < self.length {
            unsafe {
                // if ptr::read(self.ptr.add(i)) == value {
                if &*self.ptr.add(i) == &value {
                    self.swap(0, i);
                }
            }
            i += 1;
        }
        return None
    }

    pub(crate) fn binary_search(&self, value: T) -> Option<usize>
        where T: PartialOrd
    {
        let mut low: isize = 0;
        let mut high: isize = self.length as isize - 1;
        while low <= high {
            let mid: isize = (low + high)/2;
            let mid_value: &T = unsafe {&*self.ptr.add(mid as usize)};
            if &value == mid_value {
                return Some(mid as usize)
            } else if &value < mid_value {
                high = mid - 1;
            } else if &value > mid_value {
                low = mid + 1
            }
        }
        return None
    }

    pub(crate) fn recursive_binary_search(&self, low: isize, high: isize, value: T) -> Option<usize>
        where T: PartialOrd
    {
        if low <= high {
            let mid: isize = (low + high)/2;
            let mid_value: &T = unsafe {&*self.ptr.add(mid as usize)};
            if &value == mid_value {
                return Some(mid as usize)
            } else if &value < mid_value {
                return self.recursive_binary_search(low, mid - 1, value)
            } else if &value > mid_value {
                return self.recursive_binary_search(mid + 1, high, value)
            }
        }
        return None
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
        if index >= self.size {
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
        where T: Add<Output = T> + Zero + Copy + Display,
    {
        let mut sum: T = T::zero();
        for i in 0..self.length {
            println!("element={}", self[i]);
            sum = sum + *self.get(i)
        }
        return sum
    }

    // Time complexity is O(n)
    pub(crate) fn recursive_sum(&self, n: usize) -> T
        where T: Add<Output = T> + Zero + Copy,
    {
        match n.checked_sub(1) {
            Some(n_out) => self.recursive_sum(n_out) + *self.get(n),
            None => *self.get(n)
        }
    }

    // Time complexity is O(n)
    pub(crate) fn avg(&self) -> Result<T, &'static str>
        where T: Div<Output = T> + Copy + Zero + FromPrimitive + Display,
    {
        let count = T::from_usize(self.length).ok_or("Average calculation not supported for the Array data type.")?;
        return Ok(self.sum() / count)
    }

    // Time complexity is O(n)
    pub(crate) fn copy_reverse(&mut self) -> ()
        where T: Copy
    {
        let mut copy = HeapArray::with_capacity(self.size);
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

    // Time complexity is O(n)
    pub(crate) fn sorted_insert(&mut self, value: T) -> ()
        where T: Copy + PartialOrd + Display,
    {
        if self.length == self.size {
            panic!("Array is already full!");
        }
        let mut i:usize = self.length - 1;
        println!("{} > {}", *self.get(i), value);
        unsafe {
            while *self.get(i) > value {
                println!("{i}");
                ptr::write(self.ptr.add(i+1), *self.get(i));
                i -= 1;
            }
            ptr::write(self.ptr.add(i+1), value);
            self.length += 1;
        }
    }

    // Time complexity is O(n)
    pub(crate) fn is_sorted(&self) -> bool
        where T: PartialOrd
    {
        for i in 0..self.length - 1 {
            if *self.get(i) > *self.get(i+1) {
                return false
            }
        }
        return true
    }

    // TODO: I believe I should do len -2 - i in the second loop.
    // Time complexity is O(n^2)
    pub(crate) fn sort(&self) -> ()
        where T: PartialOrd + Display
    {
        for i in 0..self.length - 1 {
            for j in 0..(self.length - 1 - i) {
                if *self.get(j) > *self.get(j + 1) {
                    self.swap(j, j + 1);
                }
            }
        }
    }

    // Time complexity is ...
    pub(crate) fn signed_sort(&self) -> ()
        where T: PartialOrd + Zero
    {
        let mut i: usize = 0;
        let mut j: usize = self.length - 1;
        while i < j {
            while *self.get(i) < T::zero() {
                i += 1;
            }
            while *self.get(j) >= T::zero() {
                j -= 1;
            }
            if i < j {
                self.swap(i, j)
            }
        }
    }
}

impl<T> Default for HeapArray<T> {
    fn default() -> Self {
        HeapArray::new()
    }
}

impl<T: Clone > Clone for HeapArray<T> {
    fn clone(&self) -> Self {
        let mut clone = Self::with_capacity(self.size);
        for i in 0..self.length {
            clone.insert(i, self.get(i).clone());
        }
        clone
    }
}

impl<T: Display> Display for HeapArray<T> {
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

impl<T: Debug + Display> Debug for HeapArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", &self.get(i))?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for HeapArray<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false
        }
        for i in 0..self.length {
            if self[i] != other[i] {
                return false
            }
        }
        return true
    }

    fn ne(&self, other: &Self) -> bool {
        if self.as_bytes() == other.as_bytes(){
            return true
        }
        return false
    }
}

impl<T: PartialOrd> PartialOrd for HeapArray<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) {
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < self.length && j < other.length {
            if self[i] != other[j] {
                break
            }
            i += 1;
            j += 1;
        }
        if self[i] < other[j] {
            true
        } else {
            false
        }
    }

    fn le(&self, other: &Self) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < self.length && j < other.length {
            if self[i] != other[j] {
                break
            }
            i += 1;
            j += 1;
        }
        if self[i] <= other[j] {
            true
        } else {
            false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < self.length && j < other.length {
            if self[i] != other[j] {
                break
            }
            i += 1;
            j += 1;
        }
        if self[i] > other[j] {
            true
        } else {
            false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < self.length && j < other.length {
            if self[i] != other[j] {
                break
            }
            i += 1;
            j += 1;
        }
        if self[i] >= other[j] {
            true
        } else {
            false
        }
    }
}

impl<'a, T> IntoIterator for &'a HeapArray<T> {
    type Item = &'a T;
    type IntoIter = ArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Index<usize> for HeapArray<T> {
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

impl<T> IndexMut<usize> for HeapArray<T> {
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

impl<T> Drop for HeapArray<T> {
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

#[cfg(test)]
mod heap_array {
    use crate::structs::strings::HeapString;
    use super::*;
    use paste::paste;
    use rand::{Rng, thread_rng};

    macro_rules! define_test_new {
        ($($struct:ident<$type:ty>),*) => {
            $(
                paste::item! {
                    #[test]
                    fn [<test_new_ $struct:snake _$type >]() {
                        let array: HeapArray<$struct<$type>> = HeapArray::new();
                        assert!(array.ptr.is_null(), "Array pointer must be null!");
                        assert_eq!(array.size, 0, "Array size must be zero!");
                        assert_eq!(array.length, 0, "Array length must be zero!");
                    }
                }
            )*
        };
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_new_$type:snake >]() {
                    // fn [<test_new_$type:snake>]() {
                        let array: HeapArray<$type> = HeapArray::new();
                        assert!(array.ptr.is_null(), "Array pointer must be null!");
                        assert_eq!(array.size, 0, "Array size must be zero!");
                        assert_eq!(array.length, 0, "Array length must be zero!");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_iterator {
        ($($struct:ident<$type:ty>),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_iterator_ $struct:snake _$type:snake>]() {
                        let val_1: $struct<$type> = $struct::<$type>::default();
                        let val_2: $struct<$type> = $struct::<$type>::default();
                        let mut array: HeapArray<$struct<$type>> = HeapArray::with_capacity(2);
                        array.push(val_1.clone());
                        array.push(val_2.clone());

                        let mut iterator = array.iter();
                        assert_eq!(iterator.next().unwrap(), &val_1, "The iterator did not return the expected sequence.");
                        assert_eq!(iterator.next().unwrap(), &val_2, "The iterator did not return the expected sequence.");
                        assert_eq!(iterator.next(), None, "The iterator did not return the expected sequence.");

                        let mut into_iterator = array.into_iter();
                        assert_eq!(into_iterator.next().unwrap(), &val_1, "The into iterator impl. did not return the expected sequence.");
                        assert_eq!(into_iterator.next().unwrap(), &val_2, "The into iterator impl. did not return the expected sequence.");
                        assert_eq!(into_iterator.next(), None, "The into iterator impl. did not return the expected sequence.");
                    }
                }
            )*
        };
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_iterator_$type:snake>]() {
                        let val_1: $type = $type::default();
                        let val_2: $type = $type::default();
                        let mut array: HeapArray<$type> = HeapArray::with_capacity(2);
                        array.push(val_1.clone());
                        array.push(val_2.clone());
                        let mut iterator = array.iter();
                        assert_eq!(iterator.next().unwrap(), &val_1, "The iterator did not return the expected sequence.");
                        assert_eq!(iterator.next().unwrap(), &val_2, "The iterator did not return the expected sequence.");
                        assert_eq!(iterator.next(), None, "The iterator did not return the expected sequence.");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_with_capacity {
        ($($struct:ident<$type:ty>),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_with_capacity_ $struct:snake _$type>]() {
                        let array: HeapArray<$struct<$type>> = HeapArray::with_capacity(5);
                        assert!(!array.ptr.is_null(), "Array pointer must be null!");
                        assert_eq!(array.size, 5);
                        assert_eq!(array.length, 0);
                    }
                }
            )*
        };
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_with_capacity_$type:snake>]() {
                        let array: HeapArray<$type> = HeapArray::with_capacity(5);
                        assert!(!array.ptr.is_null(), "Array pointer must be null!");
                        assert_eq!(array.size, 5);
                        assert_eq!(array.length, 0);
                    }
                }
            )*
        };
    }

    macro_rules! define_test_values {
        ($($struct:ident<$type:ty>),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_values_ $struct:snake _$type:snake>]() {
                        let mut rng = thread_rng();
                        let values: [$struct<$type>; 2] = [rng.gen::<$struct<$type>>(), rng.gen::<$struct<$type>>()];
                        let array: HeapArray<$struct<$type>> = HeapArray::values(&values);
                        assert!(!array.ptr.is_null());
                        assert_eq!(array.size, 2);
                        assert_eq!(array.length, 2);
                    }
                }
            )*
        };
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_values_$type:snake>]() {
                        let mut rng = thread_rng();
                        let values: [$type; 2] = [rng.gen::<$type>(), rng.gen::<$type>()];
                        let array: HeapArray<$type> = HeapArray::values(&values);
                        assert!(!array.ptr.is_null());
                        assert_eq!(array.size, 2);
                        assert_eq!(array.length, 2);
                    }
                }
            )*
        };
    }

    macro_rules! define_test_resize {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_resize_$type:snake>]() {
                        let mut rng = thread_rng();
                        let values: [$type; 2] = [rng.gen::<$type>(), rng.gen::<$type>()];
                        let mut array: HeapArray<$type> = HeapArray::values(&values);
                        assert!(array.resize(10).is_ok());
                        assert_eq!(array.size, 10);

                        assert!(array.resize(5).is_ok());
                        assert_eq!(array.size, 5);

                        let err_res = array.resize(1);
                        assert!(err_res.is_err(), "Expected Panic for incorrect size!");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_push {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_push_$type:snake>]() {
                        let mut rng = thread_rng();
                        let rnd_val = rng.gen::<$type>();
                        let mut array: HeapArray<$type> = HeapArray::with_capacity(1);
                        array.push(rnd_val);
                        assert_eq!(array.length, 1, "Verifying length after push");
                        assert_eq!(array[0], rnd_val, "Verifying pushed value");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_get {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_get_$type:snake>]() {
                        let mut rng = thread_rng();
                        let rnd_val = rng.gen::<$type>();
                        let array: HeapArray<$type> = HeapArray::values(&[rnd_val]);

                        assert_eq!(array.get(0), &rnd_val, "Verifying array get method");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_delete {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_delete_$type:snake>]() {
                        let mut rng = thread_rng();
                        let rnd_val_1 = rng.gen::<$type>();
                        let rnd_val_2 = rng.gen::<$type>();
                        let mut array: HeapArray<$type> = HeapArray::values(&[rnd_val_1, rnd_val_2]);
                        array.delete(0);
                        assert_ne!(array[0], rnd_val_1, "Verifying the array element deletion");
                        assert_eq!(array[0], rnd_val_2, "Verifying the new array element at index after deletion");
                        assert_eq!(array.length, 1, "Verifying array length after deletion");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_fill {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_fill_$type:snake>]() {
                        let mut rng = thread_rng();
                        let rnd_val_1 = rng.gen::<$type>();
                        let mut array: HeapArray<$type> = HeapArray::with_capacity(5);
                        array.fill(rnd_val_1);
                        assert_eq!(array.length, array.size, "Verifying the array length after filler elements");
                        assert_eq!(array[2], rnd_val_1, "Verifying any array filler element");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_sorted_difference {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_sorted_difference_$type:snake>]() {
                        let mut rng = thread_rng();
                        let array_1_val_1: $type = rng.gen::<$type>();
                        let array_1_val_2: $type = rng.gen::<$type>();
                        let array_2_val_1: $type = rng.gen::<$type>();
                        let mut array_1: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2]);
                        let array_2: HeapArray<$type> = HeapArray::values(&[array_2_val_1, array_1_val_2]);
                        let diff_array: HeapArray<$type> = HeapArray::values(&[array_1_val_1]);
                        array_1.sort();
                        array_2.sort();
                        array_1.sorted_difference(&array_2);
                        assert_eq!(array_1, diff_array, "Testing array sorted difference");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_sorted_intersection {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_sorted_intersection_$type:snake>]() {
                        let mut rng = thread_rng();
                        let array_1_val_1: $type = rng.gen::<$type>();
                        let array_1_val_2: $type = rng.gen::<$type>();
                        let array_2_val_1: $type = rng.gen::<$type>();
                        let mut array_1: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2]);
                        let array_2: HeapArray<$type> = HeapArray::values(&[array_2_val_1, array_1_val_2]);
                        let diff_array: HeapArray<$type> = HeapArray::values(&[array_1_val_2]);
                        array_1.sort();
                        array_2.sort();
                        array_1.sorted_intersection(&array_2);
                        assert_eq!(array_1, diff_array, "Testing array sorted intersection");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_sorted_union {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_sorted_union_$type:snake>]() {
                        let mut rng = thread_rng();
                        let array_1_val_1: $type = rng.gen::<$type>();
                        let array_1_val_2: $type = rng.gen::<$type>();
                        let array_2_val_1: $type = rng.gen::<$type>();
                        let mut array_1: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2]);
                        let array_2: HeapArray<$type> = HeapArray::values(&[array_2_val_1, array_1_val_2]);
                        let diff_array: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2, array_2_val_1]);
                        diff_array.sort();
                        array_1.sort();
                        array_2.sort();
                        array_1.sorted_union(&array_2);
                        assert_eq!(array_1, diff_array, "Testing array sorted union");
                    }
                }
            )*
        };
    }

    macro_rules! define_test_sorted_merge {
        ($($type:ty),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_sorted_merge_$type:snake>]() {
                        let mut rng = thread_rng();
                        let array_1_val_1: $type = rng.gen::<$type>();
                        let array_1_val_2: $type = rng.gen::<$type>();
                        let array_2_val_1: $type = rng.gen::<$type>();
                        let mut array_1: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2]);
                        let array_2: HeapArray<$type> = HeapArray::values(&[array_2_val_1, array_1_val_2]);
                        let diff_array: HeapArray<$type> = HeapArray::values(&[array_1_val_1, array_1_val_2, array_2_val_1, array_1_val_2]);
                        diff_array.sort();
                        array_1.sort();
                        array_2.sort();
                        array_1.sorted_merge(&array_2);
                        assert_eq!(array_1, diff_array, "Testing array sorted merge");
                    }
                }
            )*
        };
    }

    #[test]
    fn test_iterator_structs() {
        #[derive(Debug)]
        #[derive(Default)]
        struct TestElement<T> {
            val: T
        }

        struct TestCollection<T> {
            items: HeapArray<TestElement<T>>
        }
        let mut items: HeapArray<TestElement<u8>> = HeapArray::with_capacity(2);
        items.push(TestElement{val: 5});
        items.push(TestElement{val: 10});
        let mut collection: TestCollection<u8> = TestCollection {
            items
        };

        // let mut iterator = collection.items.iter();
        for i in collection.items.iter() {
            println!("{:?}", i);
        }
        // assert_eq!(iterator.next().unwrap(), &val_1, "The iterator did not return the expected sequence.");
        // assert_eq!(iterator.next().unwrap(), &val_2, "The iterator did not return the expected sequence.");
        // assert_eq!(iterator.next(), None, "The iterator did not return the expected sequence.");
        //
        // let mut into_iterator = array.into_iter();
        // assert_eq!(into_iterator.next().unwrap(), &val_1, "The into iterator impl. did not return the expected sequence.");
        // assert_eq!(into_iterator.next().unwrap(), &val_2, "The into iterator impl. did not return the expected sequence.");
        // assert_eq!(into_iterator.next(), None, "The into iterator impl. did not return the expected sequence.");
    }

    define_test_iterator!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String, HeapString);
    define_test_iterator!(HeapArray<i8>);
    // define_test_iterator!(HeapArray<i8>, SparseMatrixElement<i8>);

    define_test_new!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String, HeapString);
    define_test_new!(HeapArray<i8>);

    define_test_with_capacity!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String, HeapString);
    define_test_with_capacity!(HeapArray<i8>);

    define_test_values!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    // define_test_values!(SparseMatrixElement<i8>);

    define_test_resize!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_push!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_get!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_delete!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_fill!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_sorted_difference!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_sorted_intersection!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_sorted_union!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
    define_test_sorted_merge!(char, usize, isize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
}

