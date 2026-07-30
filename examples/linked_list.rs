#![allow(dead_code)]
use crate::List::*;
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        // Nil has type `List`
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type `List`.
        Cons(elem, Box::new(self))
    }

    // Consume a list, and return the same list with a new element at its front.
}

fn main() {
    // Create a linked list.
    let list = List::new();

    list.prepend(1);
}
