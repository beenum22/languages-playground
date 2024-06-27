extern crate core;

// use std::collections::HashMap;
use std::error::Error;
// use std::fmt::format;
// use std::io;
// use std::io::{Write};
// use std::ptr::hash;
// use console_menu::{Menu, MenuOption, MenuProps};
// use terminal_menu::{menu, label, button, run, mut_menu, submenu, activate};
// use crate::MenuConfig::MenuConfig;
// use crate::MenuParam::Menu;
// use crate::structs::MyArray;
// use crate::traits::Random;
// Use Rc to avoid moving errors with references
// use std::rc::Rc;

mod adt;
mod traits;

mod structs {
    pub mod matrices;
    pub mod arrays;
    pub mod strings;
    pub mod polynomials;
    pub mod linked_lists;
    pub mod smart_ptrs;
    pub mod iterators;
}


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
// fn start_menu() {
//     let menu = menu(vec![
//
//         // label:
//         //  not selectable, useful as a title, separator, etc...
//         label("----------------------"),
//         label("Welcome to the Data Structures Demo!"),
//         label("-----------------------"),
//
//         // button:
//         //  exit the menu
//         button("Array Abstract Data Type (ADT)"),
//         button("String Data Type"),
//         // submenu("Array ADT Operations!");
//
//     ]);
//     run(&menu);
//
// }

// enum MenuConfig<'a> {
//     // Box,
//     // MenuObject(&'a Menu),
//     String(&'static str),
//     // Func(Box<dyn Fn() -> Box<dyn Any + Send>>),
//     Func(()),
//     HashMap(HashMap<&'a str, MenuConfig<'a>>),
//     Options(Vec<HashMap<&'static str, MenuConfig<'a>>>),
//     // MenuConfig,
// }

// fn placeholder() -> Result<(), Box<dyn Error>> {
//     // let foo: u16 = 10;
//     // reference_dereference(10);
//     // adt::adt_demo::<char>(10)?;
//     // adt::adt_demo::<u8>(10)?;
//
//     let mut array: MyArray<i8> = MyArray::new(10);
//     for _ in 0..10 {
//         let value: i8 = i8::random();
//         array.push(value);
//     }
//
//     let props = MenuProps {
//         fg_color: 233,
//         bg_color: 32,
//         msg_color: Some(236),
//         ..MenuProps::default()
//     };
//
//     let adt_set_ops_menu_options = vec![
//         MenuOption::new("Union Operation", move || println!("YO"))
//         //     match array.linear_search(1) {
//         //         Some(val) => println!("Value found at index={}", val),
//         //         None => println!("Value not found")
//         //     }
//         // )
//     ];
//     let mut adt_set_ops_menu = Menu::new(adt_set_ops_menu_options, MenuProps {
//         title: "Array Set Operations",
//         message: "*coffee is free!",
//         ..props
//     });
//
//     let adt_linear_search_menu_options = vec![
//         MenuOption::new(format!("Enter the value you want to look for in Array={}", array).as_str(), move || {
//             println!("Enter the value you want to look for: ");
//             let input_value = io::stdin().read_line(&mut String::new()).unwrap() as i8;
//             match array.linear_search(input_value) {
//                 Some(val) => println!("Value found at index={}", val),
//                 None => println!("Value not found")
//             };
//             foo("TOOT", adt_set_ops_menu_options, props)
//         }
//         )
//         // MenuOption::new("Linear Search", || println!("Value found at index={}", foo()))
//     ];
//     let mut adt_linear_search_menu = Menu::new(adt_linear_search_menu_options, MenuProps {
//         title: "Linear Search",
//         // message: format!("{}", array).as_str(),
//         ..props
//     });
//
//     let adt_search_menu_options = vec![
//         MenuOption::new("Linear Search", move || adt_linear_search_menu.show())
//         // MenuOption::new("Linear Search", || println!("Value found at index={}", foo()))
//     ];
//     let mut adt_search_menu = Menu::new(adt_search_menu_options, MenuProps {
//         title: "Array Search Methods",
//         message: "*coffee is free!",
//         ..props
//     });
//
//     let adt_menu_options = vec![
//         MenuOption::new("Array Search Methods", move || adt_search_menu.show()),
//         MenuOption::new("Array Set Operations", move || adt_set_ops_menu.show())
//     ];
//     let mut adt_menu = Menu::new(adt_menu_options, MenuProps {
//         title: "Array ADT Operations",
//         message: "*coffee is free!",
//         ..props
//     });
//
//     let root_menu_options = vec![
//         MenuOption::new("Array ADT Operations", move || adt_menu.show())
//         // MenuOption::new("String Type Operations", move || adt_menu.show())
//     ];
//
//     let mut menu = Menu::new(root_menu_options, MenuProps {
//         title: "Welcome to the Data Structures Demo!",
//         message: "*coffee is free!",
//         ..props
//     });
//     menu.show();
//
//     // let mut root_menu: HashMap<&str, MenuConfig> = HashMap::new();
//     // root_menu.insert("title", MenuConfig::String("Welcome to the Data Structures Demo!"));
//     // root_menu.insert("options", MenuConfig::Options(vec![
//     //     MenuConfig::HashMap()
//     // ]));
//     // root_menu.get_mut(&"s").unwrap().push(HashMap::new());
//     // // root_menu["options"].push();
//     // root_menu.insert("options", MenuConfig::Options(vec![]));
//     //
//     // let mut root_menu: HashMap<&str, MenuConfig> = HashMap::new();
//     // let mut adt_menu: HashMap<&str, MenuConfig> = HashMap::new();
//     // let string_menu: HashMap<&str, MenuConfig> = HashMap::new();
//     // let mut adt_search_menu: HashMap<&str, MenuConfig> = HashMap::new();
//     // let mut adt_set_ops_menu: HashMap<&str, MenuConfig> = HashMap::new();
//
//     // menus.insert("Array ADT Operations", )
//     //
//     // adt_search_menu.insert("Transposition Search", MenuConfig::Func(foo()));
//     // adt_search_menu.insert("Linear Search", MenuConfig::Func(foo()));
//     // adt_search_menu.insert("Binary Search", MenuConfig::Func(foo()));
//     //
//     // adt_menu.insert("Array Search Methods", MenuConfig::HashMap(adt_search_menu));
//     // adt_menu.insert("Array Set Operations", MenuConfig::HashMap(adt_search_menu));
//     //
//     // root_menu.insert("Array Abstract Data Type (ADT)", MenuConfig::HashMap(adt_menu));
//     //
//     // main.insert("Welcome to the Data Structures Demo!", MenuConfig::HashMap(root_menu));
//
//
//     // main_menu(&root_menu);
// }

fn main() -> Result<(), Box<dyn Error>>{
    // let foo: u16 = 10;
    // reference_dereference(10);
    adt::adt_demo::<char>(10)?;
    adt::adt_demo::<u8>(10)?;
    Ok(())
}
