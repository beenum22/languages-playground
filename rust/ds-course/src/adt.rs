use crate::traits::{Random};

use std::fmt::{Display};

use crate::structs::matrices::{Matrix};
use crate::structs::arrays::{HeapArray};
use crate::structs::linked_lists::LinkedList;
use crate::structs::polynomials::Polynomial;
use crate::structs::strings::{HeapString};

// fn adt_demo_with_arithmatic<T: >(size: usize) {
//     let mut array: HeapArray<T> = HeapArray::with_capacity(size);
//     for _ in 0..size {
//         let value: T = T::random();
//         array.push(value);
//     }
// }

fn adt_demo_search<T: Default + Random + PartialEq + Display + PartialOrd + Copy >(array: &HeapArray<T>) -> Result<(), &'static str> {
    let target_value: T = T::random();
    println!("Performing Iterative Binary Search on the array");
    match array.binary_search(target_value) {
        None => println!("Failed to find value={} using Iterative Binary search in the array", target_value),
        Some(val) => println!("Value={} is found at index={} using Iterative Binary search", target_value, val),
    }
    println!("---");
    println!("Performing Recursive Binary Search on the array");
    match array.recursive_binary_search(0, array.get_len() as isize - 1, target_value) {
        None => println!("Failed to find value={} using Recursive Binary search in the array", target_value),
        Some(val) => println!("Value={} is found at index={} using Recursive Binary search", target_value, val),
    }
    println!("---");
    println!("Performing Transposition Search on the array for value={}", target_value);
    match array.transposition_search(target_value) {
        None => println!("Failed to find value={} using Transposition search in the array", target_value),
        Some(val) => println!("Value={} is found at index={} using Transposition search", target_value, val),
    }
    println!("---");
    println!("Performing Move To Head Search on the array for value={}", target_value);
    match array.move_to_head_search(target_value) {
        None => println!("Failed to find value={} using Move To Head search in the array", target_value),
        Some(val) => println!("Value={} is found at index={} using Move To Head search", target_value, val),
    }
    println!("---");
    Ok(())
}

// fn adt_demo_maths<T: Default + Display + Copy + Zero + PartialEq + PartialOrd + Div<Output = T> + FromPrimitive >(array: &HeapArray<T>) -> Result<(), &'static str> {
//     println!("Maximum value in the array is {}", array.max());
//     println!("Minimum value in the array is {}", array.min());
//     println!("Sum of all the values in the array is {}", array.sum());
//     println!("Recursive sum of all the values in the array is {}", array.recursive_sum(array.length - 1));
//     println!("Average of all the values in the array is {}", array.avg()?);
//     println!("---");
//     Ok(())
// }

pub(crate) fn adt_demo<T: Default + Random + PartialEq + Display + PartialOrd + Copy >(size: usize) -> Result<HeapArray<T>, &'static str> {
    let mut array: HeapArray<T> = HeapArray::with_capacity(size);
    for _ in 0..size {
        let value: T = T::random();
        array.push(value);
    }
    println!("Array={}", array);

    println!("Reverse the order using copy method in the array");
    array.copy_reverse();
    println!("{array}");
    println!("---");

    println!("Reverse the order using swapping method in the array");
    array.swap_reverse();
    println!("{array}");
    println!("---");

    println!("Left shift/rotate the array");
    array.left_shift();
    println!("{array}");
    println!("---");

    println!("Right shift/rotate the array");
    array.right_shift();
    println!("{array}");
    println!("---");

    match array.is_sorted() {
        true => println!("Array is sorted"),
        false => {
            println!("Array is not sorted. Let's sort it.");
            array.sort();
            println!("{array}");
        },
    };
    println!("---");

    adt_demo_search(&array)?;

    println!("Create a new array of size={} and merge it to the original resized one", 10);
    let merge_array_size: usize = 5;
    let mut merge_array = HeapArray::with_capacity(merge_array_size);
    for _ in 0..merge_array_size {
        let value: T = T::random();
        merge_array.push(value);
    }
    merge_array.sort();
    println!("Array={}", array);
    println!("New array={}", merge_array);
    array.sorted_merge(&merge_array);
    println!("Merged Array={array}");
    println!("---");

    let new_size: usize = array.get_len() + 10;
    println!("Resize the Array to size={}", new_size);
    array.resize(new_size)?;
    println!("{array}");
    println!("---");

    let set_array_1 = [1, 5, 7, 8, 9, 11, 20];
    let set_array_2 = [2, 3, 7, 20, 21, 25];

    let mut merge_array_1 = HeapArray::values(&set_array_1);
    let merge_array_2 = HeapArray::values(&set_array_2);
    println!("Performing Merge Set operation for arrays: {}, {}", merge_array_1, merge_array_2);
    merge_array_1.sorted_merge(&merge_array_2);
    println!("Merge Array={}", merge_array_1);

    let mut union_array_1 = HeapArray::values(&set_array_1);
    let union_array_2 = HeapArray::values(&set_array_2);
    println!("Performing Union Set operation for arrays: {}, {}", union_array_1, union_array_2);
    union_array_1.sorted_union(&union_array_2);
    println!("Union Array={}", union_array_1);

    let mut intersection_array_1 = HeapArray::values(&set_array_1);
    let intersection_array_2 = HeapArray::values(&set_array_2);
    println!("Performing Intersection Set operation for arrays: {}, {}", intersection_array_1, intersection_array_2);
    intersection_array_1.sorted_intersection(&intersection_array_2);
    println!("Intersection Array={}", intersection_array_1);

    let mut diff_array_1 = HeapArray::values(&set_array_1);
    let diff_array_2 = HeapArray::values(&set_array_2);
    println!("Performing Difference Set operation for arrays: {}, {}", diff_array_1, diff_array_2);
    diff_array_1.sorted_difference(&diff_array_2);
    println!("Difference Array={}", diff_array_1);

    // adt_demo_maths(&array)?;

    let mut heap_string = HeapString::new("Hello World");
    println!("String={:?}", heap_string);
    heap_string.to_uppercase();
    println!("Uppercase String={}", heap_string);

    println!("Debug Array={:?}", diff_array_1);
    HeapString::new("madam").is_palindrome();

    let str_1 = HeapString::new("ABC");
    // let str_2 = str_1.clone();
    // println!("{}", str_2);

    let mut test_arr: HeapArray<char> = HeapArray::with_capacity(3);
    test_arr.push('a');
    test_arr.push('b');
    test_arr.push('c');


    println!("Target String={}", str_1);
    println!("Permutations for {} using recursion are {}", str_1, str_1.permutations_using_recursion());
    println!("Permutations for {} using swapping are {}", str_1, str_1.permutations_using_swapping());

    let mut ref_perm_arr: HeapArray<HeapString> = HeapArray::with_capacity(6);
    ref_perm_arr.push(HeapString::new("ABC"));
    ref_perm_arr.push(HeapString::new("ACB"));
    ref_perm_arr.push(HeapString::new("BAC"));
    ref_perm_arr.push(HeapString::new("BCA"));
    ref_perm_arr.push(HeapString::new("CAB"));
    ref_perm_arr.push(HeapString::new("CBA"));

    let gen_perm_arr = str_1.permutations_using_recursion();

    println!("Bytes={:?}", gen_perm_arr.as_bytes());
    println!("Bytes={:?}", ref_perm_arr.as_bytes());

    let mut matrix: Matrix<u8> = Matrix::new_sparse(5, 5);
    matrix.set(1, 2, u8::random());
    matrix.set(1, 3, u8::random());
    matrix.set(2, 1, u8::random());
    matrix.set(3, 5, u8::random());
    matrix.set(2, 5, u8::random());

    // println!("{}", matrix);
    println!("{}", matrix);

    let mut p = Polynomial::new(3);
    p.set_term(3,2);
    p.set_term(5, 1);
    p.set_term(-2, 0);
    println!("{}", p);

    {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        ll.push_front(4);

        println!("LL: {:?}", ll);
    }

    // ll.move_to_head_search(1);
    // println!("LL: {}", ll);

    // {
    //     let shared_ptr: SharedSmartPointer<u8> = SharedSmartPointer::new(1);
    //     let _cloned_shared_ptr = shared_ptr.clone();
    // }

    Ok(array)
}
