// GENERICS

use std::fmt::{Display, Debug};

// We can also use generic types with structs
struct Point<T, U> { // We can use different generics if both x and y don't have to be the same
    x: T,
    y: U,
} // This means that x and y could be the same or be different

impl<T, U> Point<T, U> {
    fn x(&self) -> &T{
        &self.x
    }
}

impl Point<f64, f64> { // This implementation block is only available to pionts with x and y of f64
    fn y(&self) -> &f64 {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<A, B>(self, other: Point<A, B>) -> Point<T, B> { // This allows us to mix types for the return value
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn get_largest<T: PartialOrd + Copy>(arr: Vec<T>) -> T { // The T specifies that the function makes use of a generic type.
    let mut largest = arr[0];
    for num in arr {
        if num > largest {largest = num}
    }
    largest
}


// TRAITS
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Default Implementation") // This defines a default implementation
    } // We declare the trait summary. Any type with this trait can implement a function called summarize.
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
// Now we implement the summary trait for newsarticle
impl Summary for NewsArticle {
    /* fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    } */
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
// Implement the summary trait for tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

// Traits as parameters
pub fn notify(item: &(impl Summary + Display)) { // Syntax sugar for notify_syntax function
    println!("Breaking news! {}", item.summarize())
}

pub fn notify_trait<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// where clause. make code cleaner
fn some_function<T, U>(x: &T, y: &U) -> impl Summary // Return value must be of a type that implements Summary
    where T: Display + Clone,
          U: Clone + Debug
{
    Tweet {
        username: String::from("Opeyemi"),
        content: String::from("Lets get rusty"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    // GENERICS
    let arr = vec![2, 3,59, 434, 902];
    println!("The largest is {}", get_largest(arr));

    let p1 = Point {x: 5, y:6};
    let p2 = Point {x: 4.2, y: 10.0};
    let p3 = Point {x: 4, y: 10.0};

    let p4 = Point {x: 5, y: 7.3};
    let p5 = Point {x: "Hello", y: 'c'};

    let p6 = p4.mixup(p5);
    println!("The new point is x: {}, y: {}", p6.x, p6.y);

    // TRAITS
    let tweet = Tweet {
        username: String::from("Opeyemi"),
        content: String::from("Lets get rusty"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Timi"),
        headline: String::from("Rust"),
        content: String::from("This is a rust article")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("News summary: {}", article.summarize());
}