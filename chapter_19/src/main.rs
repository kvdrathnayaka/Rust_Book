use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("Chapter 19...!");

    let mut num = 5;

    // raw pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // We can't dereference raw pointers outside of the unsafe block
    unsafe {
        println!("r1: {:?}, r2: {:?}", *r1, *r2);
    }

    assert_eq!(Point {x: 2, y: 0} + Point {x: 1, y: 3}, Point {x: 3, y: 3});

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {}", w);

    
}
