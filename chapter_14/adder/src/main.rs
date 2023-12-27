use add_one;

fn main() {
    let num1 = 10;
    println!("Hello, world ! {} plus one is {}", num1, add_one::add_one(num1));

    let num2 = 10;
    println!("Hello, world ! {} plus two is {}", num2, add_two::add_two(num2));
}
