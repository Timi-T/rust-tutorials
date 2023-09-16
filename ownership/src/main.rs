fn main() {
    // Three ownership rules
    //1. Each value in Rust has an owner.
    //2. There can only be one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.

    //let x = 5;
    //let y = x; // Value of x is copied to y. Copy is implemnted to integers, booleans and characters

    let s1 = String::from("hello");
    //let s2 = s1; // S1 is moved to s2 by default
    //let s2 = s1.clone(); //This clones the value of s1

    //println!("Value is: {s1}"); // S1 cannot be accessed after it has been moved to s2

    /* takes_ownership(s1); //The value of s is moved to the function
    println!("{s1}"); // s cannot be aceessed beacuse ownership has been moved */

    /* let x = 5;
    makes_copy(x);
    println!("{x}"); // X can still be accessed because a copy is created and passed to the function */

    /* let s = gives_ownership();
    println!("s = {s}"); */

    /* let s1 = gives_ownership();
    let s2 = String::from("hey!");
    let s3 = gives_and_takes_back(s2); */
    //println!("{s2}"); // We cant print out s2 beacuse the ownership has been given to s3;

    // we can use references to pass in a refference to a string rather than the string itself
    let s4 = String::from("Hello");
    let len = calculate_length(&s4); //We pass in a reference to s to avoid givin ownership to the function. References are immutable by default.
    // To make a reference mutable we simply add the mut keyword. The function still doen't take ownership in this case.
    let mut s4 = String::from("Hello");
    let len = calculate_length_mut(&mut s4);

    // You can only have one mutable reference to one variable in a scope
    let mut s5 = String::from("Hello");
    /*let r1 = &mut s5;
    let r2 = &mut s5; //This throws an error because a mutable varible for s5 has already been defined

    println!("{r1} {r2}"); */

    let r1 = &s5;
    let r2 = &s5; // multiple immutable references are allowed.
    //let r3 = &mut s5; // This throws an error because immutable references have been defined for s5 and the value is not expectd to cahnge until it has been used.

    println!("{} {}", r1, r2);

    let r3 = &mut s5; // This is fine because the values of r1 and r2 have been used for the last time in this scope and therefore modification can occur.

    //println!("{} {}", r1, r2); / This would cause an error forthe last mutable reference declared;

    // STRING SLICES
    let s6 = String::from("Hello world!");
    let hello = &s6[..5]; //start to index 5 slice
    let world = &s6[6..]; // index 6 to end slice
    let hello_world = &s6[..]; // gets a slice of the entire string which is basically apointer to the first element of the string

    // A string literal "&str" is also a string slice. i.e an immutable reference to the string in the binary.
    // Slices also work for arrays.

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() -> String { // function gives ownership of "some_string" back to the caller of the function.
    let some_string = String::from("hello");
    some_string
}

fn gives_and_takes_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize { // Passing in references to a function is called borrowing since the function isn't taking ownership of the variable
    let length = s.len();
    length
}

fn calculate_length_mut(s: &mut String) -> usize { // Here the function is able to mutate the variable without taking ownership
    let length = s.len();
    s.push_str("string");
    length
}
