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

- Creating a tuple `()`.
- Indexing a tuple `tuple.0`.
- Destructure a tuples members with `let (member1, member2) = tuple;`.
- Implemented Display Trait for Matrix structure.
- Implemented Transpose function.

## cargo run --examples arrays_and_slices

- Arrays are collection of same type objects stored contiguously in memory.
  `[i32;5]`: `[0,0,0,0,0]`.
- Initializing an array to the same value.
- Using `std::mem::size_of_val()` to examine the memory footprint of arrays.
- Using Slices to view a section of an array `[starting_index..ending_index]`.

## cargo run --example structures

- Structs are essentially named tuples, creating a custom type.
- Classic C structs have named parameters `struct Person {age:u8}`.
- Unit structs are used in generics `struct Unit;`.
- Tuple-like structs have nameless fields: `struct TupleLike{i32,u8,bool}`.
- Access fields with `.` operator `person.age`.
- Nested Destructuring in the `rect_area` activity.
- Writing functions that return structs with the `fn square` activity.

## cargo run --example enums

- Enums allow the creation of custom types by enumerating all possible variants
- Using the `match` keyword to create a callback for each invariant of an enum.

## cargo run --example type_aliases

- Referring to an enum variant via its alias. This might be useful if the enum
  name is too long.
- `type` keyword.
- `self` is an alias that is commonly used in `impl` blocks.

## cargo run --example use

- The `use` keyword will import libraries or binaries from source code others
  have written.
- `use` can also be used to adjust the scope of imports.

## cargo run --example linked_list

- Basic implementation of a linked list.
- Using `Box` to store persistent data into memory.

## cargo run --example c_like_enums

- C-like enums can also be established.
- Implicit enumerations start at 0.
- Explicit enumerations are defined with the assignment operator `=`.

## cargo run --example constants

- Rust has two different types of constants which can be declared in any scope
  including global, but type annotations are required.
- `static` is a possibly mutable variable with 'static lifetime. The static
  lifetime is inferred and does not have to be specified.
- Modifying a mutable static variable is unsafe.

## cargo run --example mutability

- By default all variables are immutable unless specified by the `mut` keyword.
- The Compiler will warn you if you misuse an immutable variable.

## cargo run --example declare_first

- All variables must be initialized before used.
- Initialize variables as close to their use as possible.

## cargo run --example freezing

- When data is bound to the same name immutably, it also freezes.
- Frozen data can't be modified until the immutable binding goes out of scope.

## cargo run --example type_casting

- Rust provides no implicit type conversion between primitives, but explicit
  type casting can be attained with use of the `as` keyword
