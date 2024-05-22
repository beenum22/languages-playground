use std::fmt::{Display, format};
use std::{mem, ptr};
use std::mem::discriminant;
use rand::Rng;
use crate::structs::MyArray;

// Trait to generate random value for the specified data type in <T>
pub(crate) trait Random {
    fn random() -> Self;
}

macro_rules! impl_random {
    ($($t:ty), *) => {
        $(
            impl Random for $t {
                fn random() -> Self {
                    rand::thread_rng().gen()
                }
            }
        )*
    };
}

impl Random for char {
    fn random() -> Self {
        // Only generate common characters
        const ASCII_MIN: u8 = 32;
        const ASCII_MAX: u8 = 126;
        rand::thread_rng().gen_range(ASCII_MIN..=ASCII_MAX) as char
    }
}

impl_random!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64, bool);

// pub(crate) trait GetPointer<T> {
//     fn get_ptr(&self) -> *const T;
// }
//
// impl<T> GetPointer<T> for T {
//     fn get_ptr(&self) -> *const T {
//         self
//     }
// }

// impl<T> GetPointer<T> for MyArray<T> {
//     fn get_ptr(&self) -> *const T {
//         // unsafe { &*(self.get_ptr()) }
//         self.get_ptr()
//     }
// }
//
// pub(crate) trait VisualizeMemory {
//     fn visualize_memory(&self) -> ();
// }
//
// impl<T: Display> VisualizeMemory for MyArray<T> {
//     fn visualize_memory(&self) -> () {
//         let ptr= self.get_ptr();
//         let value = &self;
//         println!("{:p} - {}", ptr, value);
//         // let is_pointer = mem::size_of_val(&value) == mem::size_of::<*const ()> as usize;
//         // let mut ptr_str: String =  format!(" ptr={:x} ", ptr as usize);
//         // let mut value_str: String = format!(" value={} ", self);
//         // let ptr_len: usize = ptr_str.len();
//         // let value_len: usize = value_str.len();
//         // let ref_len: usize;
//         // if ptr_len >= value_len {
//         //     ref_len = ptr_len;
//         //     value_str = format!("{}{}", value_str, " ".repeat(ptr_len - value_len))
//         // } else {
//         //     ref_len = value_len;
//         //     ptr_str = format!("{}{}", ptr_str, " ".repeat(value_len - ptr_len))
//         // }
//         // println!("┌{}┐", "─".repeat(ref_len));
//         // println!("│{}│", value_str);
//         // println!("│{}│", ptr_str);
//         // println!("└{}┘", "─".repeat(ref_len));
//         //
//         //
//         // println!("{}", is_pointer);
//         // if is_pointer == true {
//         //     unsafe {
//         //         &value.visualize_memory();
//         //     }
//         // } else {
//         //     println!("MO")
//         // }
//
//
//         // match is_pointer {
//         //     true => {
//         //         println!("Dereferenced value is another pointer");
//         //         unsafe {
//         //             // &value.visualize_memory();
//         //         }
//         //     }
//         //     false => ()
//         // }
//
//         // if *self != 0 {
//         // // if matches!(&self, _ if false) {
//         //     println!("Dereferenced value is another pointer");
//         //     // Dereference the pointer and continue checking recursively
//         //     // check_pointer(unsafe { &*self });
//         //     unsafe {
//         //         self.visualize_memory()
//         //     }
//         // } else {
//         //     println!("Dereferenced value is data: {}", self);
//         //     0 as *const T
//         // }
//
//         // let mut ptr_str: String =  format!(" ptr={:x} ", self.get_ptr() as usize);
//         // let mut value_str: String = format!(" value={} ", self.to_string());
//         // let ptr_len: usize = ptr_str.len();
//         // let value_len: usize = value_str.len();
//         // let ref_len: usize;
//         // if ptr_len >= value_len {
//         //     ref_len = ptr_len;
//         //     value_str = format!("{}{}", value_str, " ".repeat(ptr_len - value_len))
//         // } else {
//         //     ref_len = value_len;
//         //     ptr_str = format!("{}{}", ptr_str, " ".repeat(value_len - ptr_len))
//         // }
//         // println!("┌{}┐", "─".repeat(ref_len));
//         // println!("│{}│", value_str);
//         // println!("│{}│", ptr_str);
//         // println!("└{}┘", "─".repeat(ref_len));
//     }
// }
//
// impl VisualizeMemory for u8 {
//     fn visualize_memory(&self) -> () {
//         let mut ptr_str: String =  format!(" ptr={:x} ", self.get_ptr() as usize);
//         let mut value_str: String = format!(" value={} ", self.to_string());
//         let ptr_len: usize = ptr_str.len();
//         let value_len: usize = value_str.len();
//         let ref_len: usize;
//         if ptr_len >= value_len {
//             ref_len = ptr_len;
//             value_str = format!("{}{}", value_str, " ".repeat(ptr_len - value_len))
//         } else {
//             ref_len = value_len;
//             ptr_str = format!("{}{}", ptr_str, " ".repeat(value_len - ptr_len))
//         }
//         println!("┌{}┐", "─".repeat(ref_len));
//         println!("│{}│", value_str);
//         println!("│{}│", ptr_str);
//         println!("└{}┘", "─".repeat(ref_len));
//     }
// }

// impl<*const u8: Display> VisualizeMemory for *const u8 {
//     fn visualize_memory(&self) -> () {
//         let mut ptr_str: String =  format!(" ptr={:x} ", self.get_ptr() as usize);
//         let mut value_str: String = format!(" value={} ", self.to_string());
//         let ptr_len: usize = ptr_str.len();
//         let value_len: usize = value_str.len();
//         let ref_len: usize;
//         if ptr_len >= value_len {
//             ref_len = ptr_len;
//             value_str = format!("{}{}", value_str, " ".repeat(ptr_len - value_len))
//         } else {
//             ref_len = value_len;
//             ptr_str = format!("{}{}", ptr_str, " ".repeat(value_len - ptr_len))
//         }
//         println!("┌{}┐", "─".repeat(ref_len));
//         println!("│{}│", value_str);
//         println!("│{}│", ptr_str);
//         println!("└{}┘", "─".repeat(ref_len));
//     }
// }

// pub(crate) trait CountCharacters {
//     fn count_characters(&self) -> usize;
// }

// macro_rules! impl_count_chars {
//     ($($t:ty),*) => {
//         $(
//             impl CountCharacters for $t {
//                 fn count_characters(&self) -> usize {
//                     self.to_string().len()
//                 }
//             }
//         )*
//     }
// }
//
// impl_count_chars!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, char, &str, String);
