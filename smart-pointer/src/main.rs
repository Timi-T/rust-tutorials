use std::{ops::Deref, fmt::Display, rc::Rc};

enum List {
    Cons(i32, Box<List>),
    Nil,
} // We use the Box type to store smart pointers
// This way, rust stores a pointer to a list on the Heap rather than the List itself.

enum RCList {
    Cons(i32, Rc<RCList>),
    Nil,
}

struct MyBox<T: Display>(T); // Implemeting our own Box type which has a deref trait

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> { // Runs when rust deallocates memory from stack
    fn drop(&mut self) {
        print!("Dropping MyBox with data {}", &self.0);
    }
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // DEREF
    let x = 5;
    let y = Box::new(x); // same as &x
    let z = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(5, *y); // * is used to dereference y and get the value in the address it points to.
    assert_eq!(5, *z);

    let s = MyBox::new(String::from("Rust"));
    hello(&s);

    drop(z); // Frees the memory before the end of code
    println!("End of function");
    // *Box<String> -> &String, *&String -> &str

    // Reference counting. Used to count how many references there are to an item.
    let a_list = Rc::new(RCList::Cons(1, Rc::new(RCList::Cons(2, Rc::new(RCList::Nil)))));
    let b_list = RCList::Cons(3, Rc::clone(&a_list));
    let c_list = RCList::Cons(4, Rc::clone(&a_list));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
