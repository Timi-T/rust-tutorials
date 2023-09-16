use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    //panic!("Crash and burn"); // used to exit the program ungracefully.
    // We can't use the question mark ? in the main function

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() { // Create file if error kind is NOT FOUND
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => {
                    panic!("Failed to create a file: {:?}", e);
                }
            }
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        }
    };

    // We can also use unwrap and expect to get the result
    let f = File::open("hellow.txt").unwrap();
    //or
    let f = File::open("hellow.txt").expect("Failed to open file");

    // Error propagation -> Passing errors back to a function caller rather than handle it in function
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?; // the question mark returns an error if it fails

    /* let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; */

    /* match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    } */

    f.read_to_string(&mut s)?; // The question mark returns an error if it fails
    Ok(s)

    // We can also chain function calls
    /* File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s) */

    //fs::read_to_string("hello.txt"); This opens the file and returns the result as a string or an error.
}