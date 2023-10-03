pub trait Iterator {
    type Item; // This is an assoicated type. It is just like a generic type except that the ype has to be the same for all implementations
    fn next(&mut self) -> Option<Self::Item>;
}

// Default Generic type parameters and operator overloading

// Generic type parameters could specify a default concrete type
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { // We dont need to implement a concrete type for Add since it has a default Add<Rhs=Self> we would do Add<Point> if there was no default type
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // Here we have to specify the type because we are not using the default type Self (this is called overloading)
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/* trait Add<Rhs=Self> { // The Add trait has a default parameter which is Self
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
} */

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly(); // This calls the function implemented on the human type
    Pilot::fly(&person); // This calls the method from the Pilot trait
    Wizard::fly(&person); // This calls the method from the Wizard trait

    <Human as Wizard>::fly(&person); // Calls impl wizard for human

    type Kilometers = i32; // This creates a type alias.

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// Implementing trait functions with the smae name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// SUPER TRAIT
use std::fmt;

trait OutlinePrint: fmt::Display { // The fmt::Display means that any type that implements this trait must also implement the fmt::Display trait
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Pointer {
    x: i32,
    y: i32,
}

impl fmt::Display for Pointer { // Here we have to implement Display for the Pointer Struct so we can also implement OutlinePrint
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Pointer {}

// New type pattern
// Ideally, To implement a trait on a type, either the trait or the type must be defined within our crate
// This is where the new type pattern comes into play

struct Wrapper(Vec<String>); // The new type "Wrapper" Wraps the type Vec so we can implement Display on the Wrapper type

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Never type
fn bar() -> ! {
    // This means the function never returns
    panic!(""); // Panic implements the never type
}

// Dynamically sized types
// an example is str

fn dyn_size_type() {
    //let s1: str = "opeyemi"; // This doesnt work because we cant know the size of s1 at compile time
    // The solution is to use a reference to the data. The reference stores the address and length of the data during compile time.

    let s2: &str = "opeyemi"; //this works because we use a reference
}

// Advanced functions and closures in Rust

fn add_one(x: i32) -> i32 {
    x + 1
}


// Passing a function into a function
// We can also pass in a closure since functions and closures implement the Fn triat bound
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn do_twice_generic<T> (f: T, arg: i32) -> i32 // This is another way of declaring the above using trait bounds
where T: Fn(i32) -> i32 {
    f(arg) + f(arg)
}

// Returning closures from functions
fn returns_closure() -> impl Fn(i32) -> i32 { // Means that the return type must implement Fn
    |x| x + 1
}

fn returns_closure_box() -> Box<dyn Fn(i32) -> i32> { // We can use this if we want to return a closure conditionally
    Box::new(|x| x + 1)
}

/* fn main() {
    let answer = do_twice(add_one, 5); // Calling the function

    println!("The answer is: {}", answer);
} */

