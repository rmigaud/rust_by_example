// Rust provides access to a wide variety of primitives:
// Signed Integers: i8, i16, i32, i64, i128
// Unsigned Integers: u8, u16, u32, u64, u64
// Floating Point: f32, f64
// char: unicode scalar values like 'a', 'α' '∞' 4 bytes each.
// bool: true or false
// unit type: (), whose only possible value is the empty tuple: ().

// Arrays like [1, 2, 3].
// Tuples like (1, true).

// Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default.
// Integers default to i32 and floats to f64. Note that Rust can also infer types from context.
#[allow(unused)]
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;
    let a_float: f64 = 1.0; // regularly annotated.
    let an_integer: i32 = 5i32; // suffix annotation.

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 429132981984_i64;

    let mut mutable = 12; // mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    let mutable = true;
    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types and is constructed using parentheses.
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
