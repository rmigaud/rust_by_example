// cargo run --example tuples
#![allow(dead_code)]
/*
Tuples, a tuple is a collection of values of different types. Tuples are constructed using parentheses (),
and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its
members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
*/

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    //! Tuples can be used as function arguments and as return values.
    //! `let` can be used to destruct the members of a tuple into variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param) // return a struct in reverse order.
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, 5u128));

    // Tuples don't implement the Display trait by Default, so you must use Debug.
    println!("{tuple_of_tuples:?}");

    let pair = (1, true);
    println!("Pair is {pair:?}");

    println!("Reversed Pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart.
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just a Number: {:?}", (5u32));

    // Tuples can be destructured to create binding.
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{a}, {b}, {c}, {d}");

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        let result = reverse((0i32, false));
        let expected = (false, 0i32);
        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_display_test() {
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        let result = format!("{}", matrix);
        let expected = "\
        ( 1.1 1.2 )
        ( 2.1 2.2 )
        ";
        assert_eq!(result, expected);
    }

    #[test]
    fn transpose_test() {
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        let result = format!("{}", transpose(matrix));
        let expected = "Matrix:
        ( 1.1 1.2 )
        ( 2.1 2.2 )
        Transpose:
        ( 1.1 2.1 )
        ( 1.2 2.2 )";
        assert_eq!(result, expected);
    }
}

/*Activitiy

1. Recap: Add the `fmt::Display` trait to the `Matrix` struct in the above example, so that if you
switch from printing the debug format {:?} to the display format {}, you see the following output:

( 1.1 1.2 )
( 2.1 2.2 )

2. Add a `transpose` function using the reverse function as a template, which accepts a matrix as
an argument, and returns a matrix in which two elements have been swapped. For example:

println!("Matrix:\n{}", matrix);
println!("Matrix transposed:\n{}", transpose(matrix));

would result in the output:
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )*/
