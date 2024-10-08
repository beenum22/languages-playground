#![allow(dead_code)]
use crate::structs::arrays::{ArrayIterator, HeapArray};
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, slice};

pub struct HeapString {
    data: HeapArray<u8>,
}

impl HeapString {
    pub fn new() -> HeapString {
        let mut data = HeapArray::new();
        HeapString { data }
    }

    pub fn with_capacity(capacity: usize) -> HeapString {
        let mut data = HeapArray::with_capacity(capacity);
        HeapString { data }
    }

    pub fn iter(&self) -> HeapStringIterator {
        HeapStringIterator::new(self)
    }

    pub fn push(&mut self, character: char) -> () {
        if self.data.get_len() == self.data.get_size() {
            self.data
                .resize(self.data.get_size() + 2)
                .expect("Failed to resize the array"); // Resize to 2 more Bytes
        }
        let mut buffer = [0; 4];
        let encoded = character.encode_utf8(&mut buffer);
        for &byte in encoded.as_bytes() {
            self.data.push(byte)
        }
    }

    // TODO: Add test for this method
    pub fn len(&self) -> usize {
        self.data.get_len()
    }

    pub fn to_lowercase(&mut self) -> () {
        for i in 0..self.data.get_len() {
            self.data.set(i, self.data[i].to_ascii_lowercase())
        }
    }

    pub fn to_uppercase(&mut self) -> () {
        for i in 0..self.data.get_len() {
            self.data.set(i, self.data[i].to_ascii_uppercase())
        }
    }

    pub fn toggle(&mut self) -> () {
        for i in 0..self.data.get_len() {
            if self.data[i].is_ascii_lowercase() == true {
                self.data.set(i, self.data[i].to_ascii_uppercase())
            } else {
                self.data.set(i, self.data[i].to_ascii_lowercase())
            }
        }
    }

    // TODO: Check why was I doing reverse and shit?
    //TODO: We end up running two loops if we rely on Array methods. Improve later if needed.
    pub fn reverse(&mut self) -> () {
        self.data.swap_reverse();
        // self.data.left_shift()
    }

    pub fn as_bytes(&self) -> &HeapArray<u8> {
        &self.data
    }

    // TODO: Add test for this method
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts(self.data.as_ptr(), self.data.get_len());
            std::str::from_utf8(slice).expect("Invalid UTF-8 data")
        }
    }

    pub fn duplicates(&self) -> HeapArray<u8> {
        let mut map: HeapArray<usize> = HeapArray::with_capacity(128);
        for &c in &self.data {
            if (c as usize) < 128 {
                map[c as usize] += 1;
            }
        }
        let mut duplicates: HeapArray<u8> = HeapArray::with_capacity(0);
        for i in 0..map.get_len() {
            if map[i] > 0 {
                duplicates.push(self.data[map[i]])
            }
        }
        duplicates
    }

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    fn permutations_swapping(
        str: &mut HeapString,
        perm_arr: &mut HeapArray<HeapString>,
        l: usize,
        h: usize,
    ) {
        let mut i = l;

        if l == h {
            perm_arr.push(str.clone());
        } else {
            while i <= h {
                str.data.swap(l, i);
                Self::permutations_swapping(str, perm_arr, l + 1, h);
                str.data.swap(l, i);
                i += 1;
            }
        }
    }

    pub fn permutations_using_swapping(&self) -> HeapArray<HeapString> {
        let mut str_copy: HeapString = self.clone();
        let mut perm_arr: HeapArray<HeapString> =
            HeapArray::with_capacity(Self::factorial(self.data.get_len()));
        Self::permutations_swapping(&mut str_copy, &mut perm_arr, 0, self.data.get_len() - 1);
        perm_arr
    }

    fn permutations_recursion<'a>(
        &self,
        map: &mut HeapArray<u8>,
        res: &'a mut HeapArray<u8>,
        all_res: &mut HeapArray<HeapString>,
        k: usize,
    ) {
        let mut i: usize = 0;
        if k == self.data.get_len() - 1 {
            res[k] = 0;
            let mut res_str: HeapString = HeapString::with_capacity(res.get_len());
            for i in 0..res.get_len() {
                res_str.push(res[i] as char)
            }
            all_res.push(res_str);
        }
        while i != self.data.get_len() {
            if map[i] == 0 {
                res[k] = self.data[i];
                map[i] = 1;
                self.permutations_recursion(map, res, all_res, k + 1);
                map[i] = 0;
            }
            i += 1;
        }
    }

    pub fn permutations_using_recursion(&self) -> HeapArray<HeapString> {
        let k = 0;
        let mut map: HeapArray<u8> = HeapArray::with_capacity(self.data.get_len());
        let mut res: HeapArray<u8> = HeapArray::with_capacity(self.data.get_len());
        let mut all_res: HeapArray<HeapString> =
            HeapArray::with_capacity(Self::factorial(self.data.get_len()));
        self.permutations_recursion(&mut map, &mut res, &mut all_res, k);
        all_res
    }

    pub fn is_palindrome(&self) -> bool {
        let mut i: usize = 0;
        let mut j: usize = self.data.get_len() - 1;

        while i != j {
            if self.data[i] != self.data[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        if self.data[i] == self.data[j] {
            return true;
        } else {
            return false;
        }
    }

    // Time complexity is O(n)
    pub fn are_anagram(&self, other: &HeapString) -> bool {
        if self.data.get_len() != other.data.get_len() {
            return false;
        }

        let mut map: HeapArray<i8> = HeapArray::with_capacity(128);
        for &c in &self.data {
            if (c as usize) < 128 {
                map[c as usize] += 1;
            }
        }

        for &c in &other.data {
            if (c as usize) < 128 {
                map[c as usize] -= 1;
            }
            if map[c as usize] < 0 {
                return false;
            }
        }
        true
    }
}

impl From<&str> for HeapString {
    fn from(value: &str) -> Self {
        let len = value.len();
        let mut data = HeapArray::with_capacity(len);
        for &byte in value.as_bytes() {
            data.push(byte);
        }
        HeapString { data }
    }
}

impl Clone for HeapString {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl PartialEq for HeapString {
    fn eq(&self, other: &Self) -> bool {
        if self.data.get_len() != other.data.get_len() {
            return false;
        } else if self.data == other.data {
            return true;
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        if self.data != other.data {
            return true;
        }
        return false;
    }
}

impl Display for HeapString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.data.get_len() {
            write!(f, "{}", self.data[i] as char)?;
        }
        Ok(())
    }
}

impl Debug for HeapString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.data.get_len() {
            write!(f, "{}", self.data[i] as char)?;
        }
        Ok(())
    }
}

impl Default for HeapString {
    fn default() -> Self {
        HeapString::new()
    }
}

impl<'a> Iterator for HeapStringIterator<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.array_iter.next()
    }
}

impl<'a> IntoIterator for &'a HeapString {
    type Item = &'a u8;
    type IntoIter = HeapStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HeapStringIterator::new(self)
    }
}

pub struct HeapStringIterator<'a> {
    array_iter: ArrayIterator<'a, u8>,
}

impl<'a> HeapStringIterator<'a> {
    fn new(heap_string: &'a HeapString) -> Self {
        Self {
            array_iter: heap_string.data.iter(),
        }
    }
}

#[cfg(test)]
mod heap_string {
    use crate::structs::arrays::HeapArray;
    use crate::structs::strings::HeapString;

    #[test]
    fn test_new() {
        let test_str = "Hello";
        let heap_string = HeapString::from("Hello");
        assert_eq!(
            heap_string.data[0], 'H' as u8,
            "Testing first character in the array"
        );
        // assert_eq!(heap_string.data[heap_string.data.get_len() - 1], '\0' as u8, "Testing the last delimiter character in the array");
        assert_eq!(
            heap_string.data.get_len(),
            test_str.len(),
            "Testing the array length"
        );
    }

    #[test]
    fn test_push() {
        let test_str = "Hello";
        let mut heap_string = HeapString::from(test_str);
        let current_size = heap_string.data.get_size();
        heap_string.push('!');
        assert_eq!(
            heap_string.data.get_size(),
            current_size + 2,
            "Testing string array growth"
        );
    }

    #[test]
    fn test_to_lowercase() {
        let test_str = "HELLO";
        let mut heap_string = HeapString::from(test_str);
        heap_string.to_lowercase();
        assert_eq!(
            heap_string.data.as_bytes(),
            "hello".as_bytes(),
            "Testing string lowercase"
        );
    }

    #[test]
    fn test_to_uppercase() {
        let test_str = "hello";
        let mut heap_string = HeapString::from(test_str);
        heap_string.to_uppercase();
        assert_eq!(
            heap_string.data.as_bytes(),
            "HELLO".as_bytes(),
            "Testing string uppercase"
        );
    }

    #[test]
    fn test_toggle() {
        let test_str = "hello";
        let mut heap_string = HeapString::from(test_str);
        heap_string.toggle();
        assert_eq!(
            heap_string.data.as_bytes(),
            "HELLO".as_bytes(),
            "Testing string toggling"
        );
    }

    #[test]
    fn test_reverse() {
        let test_str = "hello";
        let mut heap_string = HeapString::from(test_str);
        heap_string.reverse();
        assert_eq!(
            heap_string.data.as_bytes(),
            "olleh".as_bytes(),
            "Testing string reverse"
        );
    }

    fn test_duplicates() {
        let str_1 = HeapString::from("hello");
        assert_eq!(
            str_1.duplicates(),
            HeapArray::values(&['a' as u8, 'l' as u8]),
            "Testing valid string duplicates"
        );
    }

    #[test]
    fn test_permutations_using_recursion() {
        let str_1 = HeapString::from("ABC");

        let mut ref_perm_arr: HeapArray<HeapString> = HeapArray::with_capacity(6);
        ref_perm_arr.push(HeapString::from("ABC"));
        ref_perm_arr.push(HeapString::from("ACB"));
        ref_perm_arr.push(HeapString::from("BAC"));
        ref_perm_arr.push(HeapString::from("BCA"));
        ref_perm_arr.push(HeapString::from("CAB"));
        ref_perm_arr.push(HeapString::from("CBA"));

        let gen_perm_arr = str_1.permutations_using_recursion();
        assert_eq!(
            gen_perm_arr.get_len(),
            ref_perm_arr.get_len(),
            "Verifying total permutations"
        );
        assert_eq!(
            gen_perm_arr, ref_perm_arr,
            "Verifying string permutations using recursion"
        );
    }

    #[test]
    fn test_permutations_using_swapping() {
        let str_1 = HeapString::from("ABC");

        let mut ref_perm_arr: HeapArray<HeapString> = HeapArray::with_capacity(6);
        ref_perm_arr.push(HeapString::from("ABC"));
        ref_perm_arr.push(HeapString::from("ACB"));
        ref_perm_arr.push(HeapString::from("BAC"));
        ref_perm_arr.push(HeapString::from("BCA"));
        ref_perm_arr.push(HeapString::from("CBA"));
        ref_perm_arr.push(HeapString::from("CAB"));

        let gen_perm_arr = str_1.permutations_using_swapping();
        assert_eq!(
            gen_perm_arr.get_len(),
            ref_perm_arr.get_len(),
            "Verifying total permutations"
        );
        assert_eq!(
            gen_perm_arr, ref_perm_arr,
            "Verifying string permutations using swapping"
        );
    }

    #[test]
    fn test_is_palindrome() {
        let palindrome_str = HeapString::from("madam");
        let non_palindrome_str = HeapString::from("hello");
        assert_eq!(
            palindrome_str.is_palindrome(),
            true,
            "Testing valid string palindrome"
        );
        assert_eq!(
            non_palindrome_str.is_palindrome(),
            false,
            "Testing invalid string palindrome"
        );
    }

    #[test]
    fn test_are_anagram() {
        let str_1 = HeapString::from("medical");
        let str_2 = HeapString::from("decimal");
        let str_3 = HeapString::from("decider");
        assert_eq!(
            str_1.are_anagram(&str_2),
            true,
            "Testing valid string anagram"
        );
        assert_eq!(
            str_1.are_anagram(&str_3),
            false,
            "Testing invalid string anagram"
        );
    }

    #[test]
    fn test_partialeq_trait() {
        let str_1 = HeapString::from("same");
        let str_2 = HeapString::from("same");
        let str_3 = HeapString::from("different");
        assert_eq!(
            str_1, str_2,
            "PartialEq trait equality implementation failed!"
        );
        assert_ne!(
            str_1, str_3,
            "PartialEq trait non-equality implementation failed!"
        );
    }

    #[test]
    fn test_debug_trait() {
        let str_1 = HeapString::from("foo");
        let debug_output = format!("{:?}", str_1);
        assert_eq!(debug_output, "foo", "Debug trait implementation failed!");
    }

    #[test]
    fn test_display_trait() {
        let str_1 = HeapString::from("foo");
        let display_output = format!("{}", str_1);
        assert_eq!(
            display_output, "foo",
            "Display trait implementation failed!"
        );
    }

    #[test]
    fn test_clone_trait() {
        let str_1 = HeapString::from("foo");
        let str_1_clone = str_1.clone();
        assert_eq!(str_1, str_1_clone, "Clone trait implementation failed!");
    }

    #[test]
    fn test_iterator() {
        let mut string: HeapString = HeapString::from("abc");

        let mut iterator = string.iter();
        assert_eq!(
            iterator.next().unwrap(),
            &('a' as u8),
            "The iterator did not return the expected sequence."
        );
        assert_eq!(
            iterator.next().unwrap(),
            &('b' as u8),
            "The iterator did not return the expected sequence."
        );
        assert_eq!(
            iterator.next().unwrap(),
            &('c' as u8),
            "The iterator did not return the expected sequence."
        );
        assert_eq!(
            iterator.next(),
            None,
            "The iterator did not return the expected sequence."
        );

        let mut into_iterator = string.into_iter();
        assert_eq!(
            into_iterator.next().unwrap(),
            &('a' as u8),
            "The into iterator impl. did not return the expected sequence."
        );
        assert_eq!(
            into_iterator.next().unwrap(),
            &('b' as u8),
            "The into iterator impl. did not return the expected sequence."
        );
        assert_eq!(
            into_iterator.next().unwrap(),
            &('c' as u8),
            "The into iterator impl. did not return the expected sequence."
        );
        assert_eq!(
            into_iterator.next(),
            None,
            "The into iterator impl. did not return the expected sequence."
        );
    }
}
