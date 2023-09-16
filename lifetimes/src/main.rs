// Lifetimes in structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // Rust doesn't like dangling references

    /* let r;

    {
        let x = 5;
        r = &x; // X goes out of scope after this block but r (still in scope), references x. This causes x to point to an invalid address;
    }
    println!("{r}"); */

    let s1 = String::from("hello");
    let res: &str;
    {
        let s2 = String::from("nigeria");

        res = longest(&s1, &s2);
        println!("The longest strign is {}", res); // Valid
    }
    //println!("The longest strign is {}", res); // Invalid because s2s lifetime expires after the scoped block

    // Struct Lifetimes
    let novel = String::from("Call me Ishmeal. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find string");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static life times. They live for the duration of the program
    let st: &'static str= "All string literals have a static lifetime";
}

// We can use generic lifetime annotations to specify life times of function arguments
// The lifetime of the return value is the shortest lifetime of the function arguments
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
