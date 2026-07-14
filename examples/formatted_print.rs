// cargo run --example formatted_print
use core::f64;

/// Printing is handled by a series of macros defined in `std::fmt`:
/// - `format!`: write formatted text to String.
/// - `print!`: same as `format!` but the text is printed to the console (io::stdout).
/// - `println!`: same as `print!` but with a new line.
/// - `eprint!`: same as `print!` but the text is printed to standard error (io::stderr).
/// - `eprintln!`: same as `eprint!` but with a new line.
///   All parse text in the same fashion, Rust checks formatting correctness at compile time.
///   `std::fmt`: contains many traits which govern the display of text.
/// - `fmt::Debug`: Uses the {:?} marker. Format text for debugging purposes.
/// - `fmt::Display`: Uses the {} marker. Format text in a more elegant, user friendly fashion.
///   Here we have used `fmt::Display` because the std library provides implementations for these types
///   To print text for custom types more steps are required.
///   Implementing the `fmt::Display` trait automatically implements the ToString trait.
///   [] at the end of the file print a line that says: "Pi is roughly 3.142" by formatting 3.141592
#[allow(clippy::print_literal)]
fn main() {
    // In general, the `{}` will be automatically replaced with any arguments.
    println!("July has {} days", 31);

    // Positional arguments can be used by specifying an integer inside `{}` determines which
    // additional argument will be replaced, arguments start at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over",
    );

    // Different formatting can be invoked by specifying the format character after a `:`.
    println!("Base 10:\t{}", 69420);
    println!("Base  2:\t{:b}", 69420);
    println!("Base  8:\t{:o}", 69420);
    println!("Base 16:\t{:x}", 69420);

    let number: u32 = 1;
    // You can right-justify text with a specified width. This will output "    1". (four spaces)
    println!("{number:>5}");

    // you can pad numbers and left-adjust by flipping the sign this will output "10000".
    println!("{number:<5}");
    println!("{number:0>width$}", width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`.
    // User defined types do not implement fmt::Display by default.

    #[allow(dead_code)]
    struct Structure(i32);
    // This will not compile because `Structure` does not implement fmt::Display.
    // println!("This struct `{}` won't print..", Structure(3));

    println!("{pi:.3}", pi = f64::consts::PI);
}
