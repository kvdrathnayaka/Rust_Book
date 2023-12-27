fn main() {
    println!("Chapter-3 !");

    for i in (0..5).rev() {
        println!("{:?}", i);
    }

    let x = String::from("KRX");
    let y = x.clone();
    println!("x: {:?}, y: {:?}", x, y);

    let sentence = String::from("Kavindu Randika Rathnayaka");
    let word = first_word(&sentence);

    println!("First word: {:?}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..4];

    println!("slice: {:?}", slice);

}

fn ownership(some_string: String){
    println!("Some String: {:?}", some_string);
}

fn ownership_for_integer(int: i32) {
    println!("Some integer: {:?}", int);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
