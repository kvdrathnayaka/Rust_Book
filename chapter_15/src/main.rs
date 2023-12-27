use std::ops::Deref;
use std::cell::{Ref, RefCell};

fn main() {
    println!("Chapter 15...!");
    let b = Box::new(5);

    let n: u128 = 5;
    println!("Factorial {:?}!: {:?}", n, factorial_number(&n));

    let a = 20;
    let b = Box::new(a);


}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn factorial_number<'a>(n: &u128) -> u128 {
    let mut result = 1;
    for num in 2..(n+1) {
        result *= num;
    }
    result
}
