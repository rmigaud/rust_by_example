#![allow(dead_code)]

// Implicit enumeration, values are assigned starting from 0
enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

// Explicit enumeration, values are defined directly inline.
enum Color {
    Red = 0xFF0000,
    Blue = 0x00FF00,
    Green = 0x0000FF,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("Roses are {:06x}", Color::Red as u32);
    println!("Violets are {:06x}", Color::Blue as u32);
}
