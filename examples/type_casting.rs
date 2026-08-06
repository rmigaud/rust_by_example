#![allow(unused, clippy::unnecessary_cast)]
fn main() {
    let decimal = 69.420_f32;

    // cannot implicitly convert to u8;
    // let integer: u8 = decimal; // mismatched types expected u8, found f32.

    // But we can explicitly convert with `as`
    let integer = decimal as u8;
    let character = integer as char;

    // Still some conversions are not possible:
    // let conversion_error = decimal as char; // Only u8 can be cast to char..

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T.
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type ONLY when the #![allow(overflowing_literals)]
    // lint is specified like above. Otherwise there will be a compiler error.

    // 1000 already fits in a u16
    println!("1000 as a u16 is {}.", 1000 as u16);
    // -1 does not fit so it wraps to the maximum value + 1
    println!("-1 as a u8 is {}.", -1_i8 as u8); // 255

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is {}.", 1000 % 256);

    // When casting to a signed type, the bitwise result is the same
    // as the first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    println!("300.00 as u8 is {}", 300.00 as u8); // 255, saturates at u8::MAX instead of wrapping.
    println!("-100.0 as u8 is {}", -100.0 as u8); // 0, saturates at u8::MIN instead of wrapping.
}
