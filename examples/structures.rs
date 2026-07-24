// cargo run --example structures
//
// There are three types of structs that can be created using `struct`
// Tuple structs, which are basically named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are in space.
    top_left: Point,
    bottom_right: Point,
}

#[allow(unused)]
fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 10.3, y: 0.2 };

    // Access the field of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use fields of another struct.
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        //struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    // Access the field of the point
    println!("pair contains {:?} and {:?}", integer, decimal);
}

// 1. Add a function `rect_area` which calculates the area of a `Rectangle` (try using nested destructuring).

fn rect_area(rect: Rectangle) -> f32 {
    ((rect.bottom_right.x - rect.top_left.x) * (rect.bottom_right.y - rect.top_left.y)).abs()
}
#[cfg(test)]
mod rect_tests {

    #[test]
    fn area_tests() {
        use super::*;
        let unit_rect = Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        };
        assert_eq!(rect_area(unit_rect), 0.0);

        let len_one = Rectangle {
            top_left: Point { x: -1.0, y: 1.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        };
        assert_eq!(rect_area(len_one), 1.0);

        let len_two = Rectangle {
            top_left: Point { x: -1.0, y: 1.0 },
            bottom_right: Point { x: 1.0, y: -1.0 },
        };
        assert_eq!(rect_area(len_two), 4.0);
    }
}

// 2. Add a function `square` which takes a `Point` and a f32 as arguments, and returns a `Rectangle` with
//    its top left corner on the point, and a width and height corresponding to the f32.

fn square(p: Point, n: f32) -> Rectangle {
    Rectangle {
        top_left: p.clone(),
        bottom_right: Point {
            x: p.x - n,
            y: p.y - n,
        },
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;
    #[test]
    fn square_test() {
        let pt = Point { x: 1.0, y: 2.0 };
        let size = 2.0;
        assert_eq!(
            square(pt.clone(), size),
            Rectangle {
                top_left: pt.clone(),
                bottom_right: Point {
                    x: &pt.x - size,
                    y: &pt.y - size
                }
            }
        );
        assert_eq!(rect_area(square(pt.clone(), size)), size * size);
    }
}
