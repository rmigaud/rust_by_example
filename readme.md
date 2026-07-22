# Rust by Example

To run an example use the command `cargo run --example {example_name}`.

## cargo run --example helloworld

- comments
- documentation-comments
- functions
- `let` variable binding
- Compiler annotations

## cargo run --example formatted_print

- Formatting macros: print, println, eprint, eprintln.
- Printing variables by name and by argument position.
- Formatting numbers in binary, octal, and hexadecimal representations.
- The Display Trait.
- assert_eq! macro

## cargo run --example debug

- All types can derive the Debug Trait (not true for Display trait).
- Pretty printing with `{:#}`.
- Custom types with structs.

## cargo run --example display

- Import via `use` statement.
- Using `impl` to implement the `fmt::Display` trait.
- The `write!` macro.

## cargo run --example testcase_list

- std::collections::LinkedList;
- LinkedList, push_back() method.
- if-else statements

## cargo run --example formatting

- This formatting functionality is implemented via traits and there is one trait
  for each argument type.
- Casting primitive types.
- Configure tests module with `#[cfg(tests)] mod tests {}` block.

## cargo run --example primitives

- Signed Integers: i8, i16, i32, i64, i128
- Unsigned Integers: u8, u16, u32, u64, u64
- Floating Point: f32, f64
- char: unicode scalar values like 'a', 'α' '∞' 4 bytes each.
- bool: true or false
- unit type: (), whose only possible value is the empty tuple: ().

## cargo run --example literals_and_operators

- Infixing type annotation to the end of a number `42u8`.
- Scientific Notation representing Ten-Thousand `10e3`.
- Underscores for readability One Million = `1_000_000`.
- Logical comparisons such as `true && false`: `false`
- Bitwise operations such as `0101 & 0110`: `0100`

## cargo run --example tuples
