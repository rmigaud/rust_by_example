// This is a comment, you can run this example one directory higher by issuing the command:
// `cargo run --example helloworld`

/// This is a documentation comment for the main function
fn main() {
    // `let` binds a value to a variable, it is immutable unless `mut` is specified.
    let greeting: String = greeting(String::from("world"));
    // convert the str reference "world" to a heap memory String and pass it to `greeting`,
    // store the result in the variable greeting.

    println!("{greeting}"); // `println!` is a macro (denoted by !), that prints to the console
}

// `fn` Defines a function with a signature fn_name(parameter:Type) -> ReturnType {..}
pub fn greeting(name: String) -> String {
    //! This is a documentation-comment for the enclosing item `greeting(name)`
    // `format!` is a macro (denoted by !), that returns a string instead of printing.
    format!("hello, {}", name) // Note: no semicolon means return this statement.
}
