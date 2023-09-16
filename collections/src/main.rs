use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3]; // An array is stored in stack since the size is always known
    let mut v: Vec<i32> = Vec::new(); // We define a vector that can be stored in the heap since the size is unknown at compile time
    v.push(1);
    v.push(2);
    v.push(3); // We use .push to add elements to the vector

    // We can also declare a vector using the vec macro
    let mut v2 = vec![1, 2, 3, 4, 5];

    let third_element = &v2[2];
    //println!("Third element is {}", third_element);

    // We use a get method on the vector to get an element which prevents runtime errors
    match v2.get(2) {
        Some(element) => println!("Third element is {}", element),
        None => println!("Index is out of range")
    }

    // Iterating through elements of a vector
    for ele in &v2 {
        println!("{ele}")
    }

    // We can mutate the the elements of the vectors also
    for ele in &mut v2 {
        *ele += 50; // we dereference the element and mutate its value
    }

    // We can use enums to store different data types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let excel_vector = vec![
        SpreadsheetCell::Int(30),
        SpreadsheetCell::Float(32.4),
        SpreadsheetCell::Text(String::from("40"))
    ];

    match &excel_vector[0] {
        SpreadsheetCell::Int(val) => println!("Number: {}", val),
        _ => println!("Not an integer"),
    }

    // STRINGS!!!
    // Strings are stored as a collection of utf-8 encoded bytes;
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2; // we can add 2 strings together by doing this. ownership of s1 is passed to s3 and characters of s2 are copied into s3

    let hello = String::from("Hello");
    //let c = hello[0]; // This cannot woork because utf-8 characters are between 1-4 bytes. for characters that are more than 1 byte, indexing won't work for them.
    // A string can consist of 

    // 1. Bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in "नमस्ते".bytes() {
        println!("{b}")
    }

    // 2. scalar values... building blocks for characters: ['न', 'म', 'स', '्', 'त', 'े']
    for c in "नमस्ते".chars() {
        println!("{c}")
    }

    // 3. grapheme clusters... Actual characters: ["न", "म", "स्", "ते"]
    // For grapheme clusters, we need to import a crate to be able to print it out
    for g in "नमस्ते".graphemes(true) {
        println!("{g}")
    }

    // HASHMAPS
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);

    match team_score {
        Some(score) => println!("The score is: {}", score),
        None => println!("Team doesn't exist!")
    }

    // Iterate through a hashmap
    for (k, v) in &scores {
        println!("Team {} has a score of {}", k, v);
    }

    // updating values
    scores.insert(String::from("Blue"), 20); //overwrites value of blue

    scores.entry(String::from("Yellow")).or_insert(100); // inserts key Yellow into the hasmap if it doesn't exist already

    // string manipulation
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
