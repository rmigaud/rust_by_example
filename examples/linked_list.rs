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
        //! `new` returns the empty `Nil` enum invariant of type `List`.
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        //! Consume a list and return the same list with a new element at its front,
        //! `Cons` also has type `List`.
        Cons(elem, Box::new(self))
    }

    // Return the length of the list.
    fn len(&self) -> u32 {
        // here we're taking a reference to self instead of consuming the list.
        match self {
            Cons(_, list) => 1 + list.len(),
            Nil => 0,
        }
    }

    // Return string of list.
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "nil".to_string(),
        }
    }
}

fn main() {
    // Create a linked list.
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show final list
    println!(
        "list length: {},\n list contents: {}",
        list.len(),
        list.stringify()
    );
}
