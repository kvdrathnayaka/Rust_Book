use std::any::Any;

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn _square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
}

enum IPs {
    IPv4,
    IPv6
}

enum Option<T> {
    None,
    Some(T)
}

fn main() {
    println!("Chapter-5 !");
    let rect1 = Rectangle {
        width: 20,
        height: 30
    };
    let _rect2 = Rectangle {
        width: 10,
        height: 30
    };
    let _rect3 = Rectangle {
        width: 50,
        height: 60
    };

    println!("Can rect1 hold rect2 ? {:?}", rect1.area());

    let ipv4 = IPs::IPv4;
    let ipv6 = IPs::IPv6;

    let some_num = Some(5);


}
