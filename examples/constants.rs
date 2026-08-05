// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    println!("Welcome to the {} programming language.", LANGUAGE);
    let my_var: i32 = 5;
    println!(
        "{} is {} the threshold.",
        my_var,
        if is_big(my_var) { "above" } else { "below" }
    );
}
