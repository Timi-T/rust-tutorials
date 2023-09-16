#[derive(Debug)]
struct Rectangle {
    height: u32,
    width:u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new (val: i32) -> Guess {
        if val < 1 {
            panic!("Guess must be greater than or equal to 1, Got {}", val);
        } else if val > 100 {
            panic!("Guess must be less than or equal to 100, Got {}", val);
        }
        Guess { value: val }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("The value is {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

   /*  #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failing_test() {
        panic!("Make test fail");
    } */

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 50,
            width: 70,
        };
        let smaller = Rectangle {
            height: 40,
            width: 50
        };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 50,
            width: 70,
        };
        let smaller = Rectangle {
            height: 40,
            width: 50
        };
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(5), 7);
        assert_ne!(add_two(6), 9);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Opeyemi");
        assert!(
            result.contains("Opeyemi"),
            "Greeting did not contain name, value was `{}`", // Error message if test fails
            result // Error message arguments
        );
    }

    #[test]
    #[should_panic (expected = "Guess must be less than or equal to 100, Got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn result_type() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Failed"))
        }
    }

    #[test]
    #[ignore]
    fn show_stdout() {
        assert!(prints_and_returns_10(20) == 10);
    }
}

// COOMAND LINE OPTIIONS
// -- --test-threads=1. Runs all tests in 1 thread. This can be useful when tests modify a file on disk.
// standard output is captured for passing tests but is displayed for failing tests. -- --show=-output is used to display stdout.
// Run a specific test by specifying the name
// We can run a subset of tests that start with or contains a substring
// We can run tests based on the module by specifying module_name::
// We add the #[ignore] attribute to ignore tests
// To run only ognored tests we use -- --ignored
// Unit tests live in the same file as product code.