// cargo run --example arrays_and_slices
#![allow(unused)]
/*
An Array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created
using brackets `[]`, and their length, which is known at compile time, is part of their type
signature [T;length].

Slices are similar to arrays, but their length is not known at compile time. Instead, a lice is a
two-word object; the first word is a pointer to the data, the second word is the length of the slice.
The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64.
Slices can be used to borrow a section of an array and have the type signature &[T].
*/
pub fn analyze_slice(slice: &[i32]) {
    println!(
        "This slices' first element is: {}\nIt has a length of: {}\nand a memory size of: {}",
        slice[0],
        slice.len(),
        std::mem::size_of_val(slice)
    );
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs);

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", ys[0]);
    println!("Second element of the array: {}", ys[1]);

    // `len` return the count of elements in the array.
    println!("Number of elements in array: {}", ys.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", std::mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slice can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice
    // `ending_index` is one more than the last position in the slice
    analyze_slice(&ys[1..4]);

    // Example of the empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose.

    // Arrays can be safely accessed using `.get`, which returns an `Option`. This can be matched
    // as shown below, or used with `.expect()` if you would like the program to exit with a nice message
    // instead of happily continue.
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{i}: {xval}"),
            None => println!("Slow down! {} is too far!", i),
        }
    }
    // Out of bound indexing on array with constant value causes compile time error.
    // println!("{},xs[5]");
    // Out of bound indexing on slice causes runtime error.
    // println!("{}", xs[..][5]);
}
