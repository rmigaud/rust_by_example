#![allow(unused, dead_code)]
use std::collections::LinkedList;
use std::fmt;
use std::fmt::{Display, Formatter};
struct LinkedListDisplayWrapper {
    list: LinkedList<i32>,
}
impl Display for LinkedListDisplayWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.list.len() < 5 {
            write!(f, "{:?}", self.list)
        } else {
            write!(f, "{:#?}", self.list)
        }
    }
}

fn main() {
    let list = LinkedListDisplayWrapper {
        list: LinkedList::from([1, 2, 3, 4, 5, 6, 7]),
    };
    println!("{list}");
}

#[cfg(test)]
mod tests {
    // cargo test --example testcase_list
    use super::*;

    #[test]
    fn add_list_test() {
        let mut result = LinkedList::new();
        result.push_back("one");
        result.push_back("two");
        let expected = LinkedList::from(["one", "two"]);
        assert_eq!(expected, result);
    }
}
