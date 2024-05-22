use crate::traits::{Random};
use crate::structs::{MyArray};

use std::fmt::{Display};
use std::ops::Index;

pub(crate) fn adt_demo<T: Random + Default + Display + PartialEq + PartialOrd>(size: usize) -> MyArray<T> {
    let mut array: MyArray<T> = MyArray::new(size);
    for _ in 0..size {
        let value: T = T::random();
        array.push(value);
    }
    let x: u8 = 10;
    let x_ptr = &x;

    // println!("Array={}", array);
    // println!("Array Stack Address={:p}", &array);
    // println!("Array Heap Address={:p}", array.get_ptr());
    // println!("Pointer to u8: {:p}", &x);
    // println!("Pointer to u8: {:p}", x.get_ptr());
    // x.visualize_memory();
    // array.visualize_memory();

    let mut ds_array = MyArray::new(10);
    ds_array.push(1);
    ds_array.push(2);
    ds_array.push(5);
    ds_array.push(7);
    ds_array.push(9);
    ds_array.push(10);
    ds_array.push(15);
    println!("{}", ds_array);
    // ds_array.insert(4, 20);
    // println!("{}", ds_array);
    // ds_array.delete(3);
    // println!("{}", ds_array);
    let target_value= 10;
    // match ds_array.transposition_search(target_value) {
    //     None => println!("Key={} doesn't exist in the Array", target_value),
    //     Some(val) => println!("Key={} is at index={}", target_value, val),
    // }
    // println!("{}", ds_array);
    //
    // match ds_array.move_to_head_search(target_value) {
    //     None => println!("Key={} doesn't exist in the Array", target_value),
    //     Some(val) => println!("Key={} is at index={}", target_value, val),
    // }
    // println!("{}", ds_array);

    println!("Performing Iterative Binary Search on the Array={}", ds_array);
    match ds_array.binary_search(target_value) {
        None => println!("Value={} doesn't exist in the Array", target_value),
        Some(val) => println!("Value={} is at index={}", target_value, val),
    }
    println!("{}", ds_array);

    println!("Performing Recursive Binary Search on the Array={}", ds_array);
    match ds_array.binary_search(target_value) {
        None => println!("Value={} doesn't exist in the Array", target_value),
        Some(val) => println!("Value={} is at index={}", target_value, val),
    }
    println!("{}", ds_array);

    let set_index: usize = 5;
    let set_value = 100;
    println!("Set a value={} at particular index={} in the Array", set_value, set_index);
    ds_array.set(set_index, set_value);
    println!("{}", ds_array);

    println!("Maximum value in the array is {}", ds_array.max());
    println!("Minimum value in the array is {}", ds_array.min());
    println!("Sum of all the values in the array is {}", ds_array.sum());
    println!("Recursive sum of all the values in the array is {}", ds_array.recursive_sum(ds_array.length - 1));
    println!("Average of all the values in the array is {}", ds_array.avg());

    let get_opt_index: usize = 200;
    match ds_array.get_optional(get_opt_index) {
        Some(val) => println!("Get (optional) at index={} is value={}", get_opt_index, val),
        None => println!("Get (optional) at index={} is value=None", get_opt_index)
    }

    println!("Reverse the order using copy method in the array");
    ds_array.copy_reverse();
    println!("{ds_array}");

    println!("Reverse the order using swapping method in the array");
    ds_array.swap_reverse();
    println!("{ds_array}");

    println!("Left shift/rotate the array");
    ds_array.left_shift();
    println!("{ds_array}");

    println!("Right shift/rotate the array");
    ds_array.right_shift();
    println!("{ds_array}");

    // match ds_array.transposition_search(10) {
    //     None => println!("Key={} doesn't exist in the Array", 10),
    //     Some(val) => println!("Key={} is at index={}", 10, val),
    // }
    // println!("{}", ds_array);

    // let arr_ptr = array.get_ptr();
    // let arr_ref = array.get_ref();
    // let arr_s_ref = &array;
    // // println!("Pointer to array: {:p}", &array);
    // println!("Pointer to array: {:p}", arr_ptr);
    // println!("Ref to array: {:p}", arr_ref);
    // println!("Simple Ref to array: {:p}", arr_s_ref);
    // unsafe {
    //     println!("array: {}", *array.get_ptr());
    // }
    // println!("u8 at pointer: {}", *x_ptr);
    // println!("{:p}", array.get_ptr());
    // println!("{:p}", &x);
    // let v = VisualizeDataType::init(&array);
    // println!("{}", v);
    // let int_data = VisualizeDataType::init(&x);
    // let array_data = VisualizeDataType::init(&array);
    // println!("{}", VisualizeDataType::init(&x));
    // println!("{:p}", VisualizeMemory::init(array).ptr);
    // println!("{}", VisualizeDataType::init(&array).value);
    return array;
}
