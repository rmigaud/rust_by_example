#![allow(clippy::needless_late_init)]
fn main() {
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding, another_binding must be initialized before use.
    // println!("another binding {}", another_binding);
    another_binding = 1;
    println!("another binding: {another_binding}");
}
