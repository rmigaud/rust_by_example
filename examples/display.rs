#![allow(dead_code, unused)]
// cargo run --example display

// Import (via `use`) the `fmt` module.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented.
// This is a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented manually.
impl fmt::Display for Structure {
    // This trait required `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.0` to refer to each positional data point.
        write!(f, "({})", self.0)
    }
}

// A structure holding two numbers.
// `Debug` will be derived so the result can be contrasted with `Display`
#[derive(Debug)]
struct Point(i64, i64);

// Implement Display for our struct, same as before this is a common pattern.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.0, self.1)
    }
}

fn main() {
    let mm: Point = Point(1, 2);
    println!("{mm}");
}

#[cfg(test)]
mod tests {
    // cargo test --example display
    use super::*;
    #[test]
    fn display_test() {
        assert_eq!(format!("{}", Structure(1)), "(1)");
        assert_eq!(format!("{}", Point(1, 2)), "{1, 2}")
    }
}
