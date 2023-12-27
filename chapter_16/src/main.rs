#[path = "blog/post.rs"] mod post;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use chapter_16::{ Draw, Button, Screen};
use crate::post::Post;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw select box from chapter_16::Draw...!");
    }
}

struct Point {
    x: i32,
    y: i32
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Chapter 16...!");
    let handle = thread::spawn(||{
        for i in 1..5 {
            println!("Hi number [{:?}] from spawn thread...", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..2 {
        println!("Hi number [{:?}] from main thread...", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi...!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok")
            })
        ]
    };
    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let p = Point { x: 0, y: 7};

    let Point { x, y } = p;
    println!("{}, {}", x, y);


}


