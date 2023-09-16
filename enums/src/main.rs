enum IpAddrKind {
    v4(u8, u8, u8, u8),
    v6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implement Methods and associated functions on enums
impl Message {
    fn some_function() {
        println!("Let's get rusty");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// OPTION ENUM
// In rust there are no null values. instead we have the option enum
/* enum Option<T> {
    Some(T),
    None
} */

#[derive(Debug)]
enum UsState {
    Lagos,
    Abia,
    Oyo,
    Osun,
    Edo,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let localhost = IpAddrKind::v4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_string = Some("a string");
    let null_number: Option<i32> = None; // To use None, the type must be explicitly declared

    let x: i8= 5;
    let y = Some(4);

    //let sum = x + y; // This doesn't work because they are different types
    let sum = x + y.unwrap_or(0); // y defaults to 0 when it is none.
    value_in_cents(Coin::Quarter(UsState::Lagos));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 5,
        Coin::Nickel => 10,
        Coin::Dime => 20,
        Coin::Quarter(state) => {
            println!("This is {:?} state!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None, // Every other arm of the expression is set to none
    }
}
