use std::collections::HashMap;

fn pigify(word: &str) -> String {
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();
    let low_first_letter = first_letter.to_lowercase().next().unwrap();

    match low_first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay ", word),
        _ => format!("{}-{}ay ", letters.as_str(), low_first_letter)
    }
}

fn main() {
    println!("Chapter 8...!");

    // Vector
    let mut vec: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    vec.push(1);
    v.push(6);

    println!("vec: {:?}, v: {:?}", vec, v);

    match v.get(2) {
        Some(third) => println!("The third element is: {:?}", third),
        None => println!("There is no third element...!")
    }

    for ele in &mut v {
        *ele += 1;
        println!("elements: {:?}", ele);
    }

    // String
    let mut s = String::from("Kavindu");
    s.push_str(" Randika");
    println!("s: {:?}", s);

    let mut s1 = String::from("KRX ");
    let s2 = "Randika";
    s1.push_str(s2);
    println!("s1: {:?}, s2: {:?}", s1, s2);

    // Hash Map
    let mut vec = vec![1, 2, 3, 4, 5, 2, 8, 6 ,1, 2, 7, 9, 5];
    vec.sort();
    let median = vec.get((vec.len()-1)/2);
    match median {
        Some(val) => println!("The Median: {:?}", val),
        None => println!("There is no median value...!")
    }

    let mut map = HashMap::new();
    for val in vec {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    let key_with_max_val = map.iter().max_by_key(|entry| entry.1).unwrap();
    println!("The Mode: {:#?}", key_with_max_val.0);

    let sentence = String::from("I Love you");
    let words = sentence.trim().split_whitespace();
    let mut pig_latin = String::new();
    for word in words {
        pig_latin.push_str(&*pigify(word));
    }
    println!("Pig Latin translation({}): {}", sentence, pig_latin);

    let mut hash_map = HashMap::new();
    hash_map.insert("Sally", "Engineering");
    hash_map.insert("Amir", "Sales");
    hash_map.insert("John", "Engineering");
    hash_map.insert("Roman", "Engineering");
    hash_map.insert("Molly", "Sales");

    let dept = "Engineering";

    let mut filter_by_dept: HashMap<String, Vec<String>> = HashMap::new();
}
