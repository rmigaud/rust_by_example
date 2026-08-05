#[allow(unused)]
fn main() {
    // This binding lives within the `main` function
    let long_lived_binding = true;

    // This code block constrains variables to a smaller scope
    {
        // from here,
        let short_lived_value = 1;
    } // to here.
    // By this point, `short_lived_value` can no longer be referenced.
    // yet, long_lived_binding is still a valid reference.
    println!("Is first binding still valid? {long_lived_binding}");
}
