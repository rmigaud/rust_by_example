#![allow(unused_mut)]
fn main() {
    let mut mutable_int = 7_i32;
    {
        // Shadowing by immutable
        let _mutable_int = mutable_int;

        // Now _mutable int cannot by changed, it is "frozen"
        // _mutable_int = 50; // panic!
    }
    // Now we can change mutable_int again!
    let _mutable_int = 50;
    println!("{_mutable_int}");
}
