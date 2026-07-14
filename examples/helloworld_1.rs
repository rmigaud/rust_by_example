// This is a comment, you can run this example by issuing the command:
// `cargo run --example helloworld_1`

/// This is a documentation comment for the main function
fn main() {
    // let binds a value to a variable, it is immutable unless `mut` is specified.
    let greeting: String = greeting(String::from("world"));
    println!("{greeting}");
}

// fn defines a function and -> specifies it's return type
pub fn greeting(name: String) -> String {
    //! This is documentation for the enclosing item `greeting()`
    format!("hello, {}", name) // format is a macro (denoted by !), returns a string without the ;
}
