mod adt;
mod traits;
mod structs;

// use std::mem;

// fn reference_dereference(data: u16) {
//     let data_ptr: *const u16 = &data;
//     println!("Raw pointer reference in Stack to data={:p}", data_ptr);
//     println!("data reference raw pointer's address in Stack={:p}", &data_ptr);
//     unsafe {println!("Raw pointer unsafe dereference to data={}", *data_ptr)};
//     println!("Dereference of reference to data={}", *&data);
//
//     let width = 5;
//     let height = 3;
//     let default_ptr_len: usize = mem::size_of::<*const u16>() * 2;
//     // Draw the top of the box
//     println!("┌{}┐", "─".repeat(default_ptr_len));
//     // println!("│{}│", " ".repeat(default_ptr_len));
//     println!("│ data_ptr{}│", " ".repeat(default_ptr_len - "data_ptr".len() - 1));
//     println!("│ ptr={:p} │", data_ptr);
//     println!("│ addr={:p} │", &data_ptr);
//     // println!("│{}│", " ".repeat(default_ptr_len));
//     println!("└{}┘", "─".repeat(default_ptr_len));
//     // Draw the sides of the box
//     // for _ in 0..height {
//         // println!("│{}│", " ".repeat(width));
//     // }
//     // Draw the bottom of the box
//     // println!("└{}┘", "─".repeat(width));
//
//     // println!("{}", mem::size_of::<*const u16>());
//     println!("\
//         |data_ptr|\n\
//         |ptr={:p}| ---> |data={}|\n\
//         |addr={:p}|
//         ", data_ptr, data, &data_ptr);
// }

fn main() {
    // let foo: u16 = 10;
    // reference_dereference(10);
    adt::adt_demo::<char>(10);
}
