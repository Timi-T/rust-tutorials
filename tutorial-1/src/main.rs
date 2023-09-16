use std::io;

fn main() {
    // VARIABLES, CONSTANTS AND SHADOWING
    /* ==================================== */
    /* let mut x = 4; //the mut keyword must be added to x to allow changed in the future
    println!("x is : {}", x);

    x = 7; // A can be changed because it has "mut" when defined, but it must be changed to a value of the same type

    {
        let x = x - 3; // X defined here does not affect the x defined outside the scope. X from outside can be accessed but not mutated
        println!("x is : {}", x);
    }

    let x = x + 5;
    println!("x is : {}", x);
    const SEC: u32 = 60; // Cannot be changed or redefined, Must be type anotated
    println!("{}", SEC); // Println must recieve a string literal */
    /* ==================================== */

    // DATA TYPES (PRIMITIVE)
    /* ==================================== */
    /* // Types of primitive
    // - Scalar -> single value
    let x: i32 = 2; // i32 is the default value for an integer in rust
    // Others include i8 i16 i32 i64 i128
    let y: f32 = 3.2; // Floating point values include f32 and f64 with f64 being the default
    // Boolean
    let true_or_false: bool = false; // bool can be true/1 or false/0
    // Character
    let letter: char = 'f'; //used to represent single characters

    // - compound -> multiple values (tuples & arrays)
    let mut tup1: (u32, bool, char) = (8, true, 'g'); //A tuple's type is defined by the characters it stores
    let tup2: (u8, bool, char) = (8, true, 'g'); // This is different from tup1 because u8 is different from u32
    //tuples are immutable by default and can be made mutable by using "mut" keyword
    tup1 = (80, false, 'y');
    println!("{}", tup1.0);

    // Arrays
    let mut arr1: [i32; 6] = [1,2,3,4,5];
    arr1[4] = 10;
    println!("{}", arr1[4]) */
    /* ==================================== */

    // USER INPUT
    /* ==================================== */
    /* // Preludes are things imported into every rust program automatically e.g println! fn
    // For collecting user input, we have to manually import the library/crate/package
    // -- A module is a specific piece of functionality from a crate

    println!("Enter value...");
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    println!("{}", user_input); */
    /* ==================================== */

    // ARITHMETIC AND TYPE CASTING
    /* ==================================== */
    /* let x: u8 = 9; // 0 - 255
    let y: i8 = 10; // -128 - 127
    let z = x + y; // We can't add different types together */

    /* let a: u8 = 255;
    let b: u8 = 3;
    let c = a + b; // This would throw an error in compile time because the addition of both values doesn't fit into the range of u8. basically compile prevents an overflow in runtime */

    /* let d = 255_000.0_f32; // typecasting
    let e = 10.0f64; // typecasting
    let f = 5 as u8; // typecasting

    let g = d % (e as f32); // typecasting to perform arithmetic
    println!("{}", g); */

    // We can use underscore to make our numbers more readable e.g 127_000 = 127000
    // ADVICE!!! Always cast smaller type to larger type and not the other way round to avoid overflows

    // Manipulate input from commandline

    /* let mut user_input = String::new();
    println!("Enter a value");
    io::stdin().read_line(&mut user_input).expect("Failed to load input for some reason");

    let int_user_input: i64 =  user_input.trim().parse().unwrap();
    // .trim() removes the new line character when we press enter
    //.parse() converts the string to an integer if possible as specified in the variable declaration (int_user_input: i64)
    // .unwrap() converts the parsed result to its actual type

    println!("{}", int_user_input +5); */
    /* ==================================== */

    // CONDITION AND CONTROL FLOW (IF/ELSE)
    /* ==================================== */
    /* let cond = 2_f32 <=2.2; // you cant compare 2 different types directly
    // Logical operators are && || !
    println!("{}", cond); */
    /* ==================================== */
    let mut a = 5;
    println!("a before change: {}", a);
    change_num(a);
    println!("a after change: {}", a);

    /* let ans = add_nums(20, 30);
    println!("{}", ans) */
}

fn add_nums(x: i32, y: i32) -> i32 { //The function can be defined anywhere in the file. it doesnt cascade while compiling
    println!("The sum is: {}", x + y);

    let numb: i32 = { // The values in the braces result to an expression
        let a: i32 = 9;
        a + 1 // This is returned because there is no semicolon
    };

    println!("{}", numb);

    return x + y;
}

fn change_num (mut x: i32) {
    x = 999;
}