/*
## Debug

All types which want to use `std::fmt` formatting traits require an
implementation to be printable. Automatic implementations are only
provided for types such as in the std library. All other _must_ be
manually implemented somehow. The `fmt::Debug` trait makes this very
straightforward. All types can `derive` (automatically create) the
`fmt::debug`
*/
#![allow(unused)]
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable.
#[derive(Debug)]
struct Structure(i32);

// Put Structure inside of the structure `Deep` make it printable also.
#[derive(Debug)]
struct Deep(Structure);

// Using #? will pretty print the debug fmt
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?}", Structure(1));

    println!("{:?}", Deep(Structure(2)));

    println!(
        "{:#?}",
        Person {
            name: "Ferris",
            age: 20
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn debug_derived_struct_prints() {
        println!("{:?}", Structure(2));
    }
}
