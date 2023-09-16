struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // Debug is a trait
struct Rectangle {
    heigth: u32,
    width: u32,
}

// Implement methods on a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

//Implementing associated functions.
// Associated functions are not tied to the instance of the struct so the self attribute is not passed
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { heigth: size, width: size }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("ope"),
        email: String::from("ope@gmail.com"),
        sign_in_count: 1,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("timi");

    println!("{name} {}", user1.username);

    let user2 = build_user(String::from("timi@gmail.com"), String::from("ope"));

    let user3 = User {
        email: String::from("lase@gmail.com"),
        username: String::from("lase"),
        ..user2
    };

    // Create structs with tuple fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle { heigth: 30, width: 50 };

    let rect1 = Rectangle {heigth: 20, width: 40};
    let rect2 = Rectangle {heigth: 40, width: 40};

    let rect3 = Rectangle::square(50);

    println!("{} {}", rect.can_hold(&rect1), rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!("Area of rectangle is {} square pixels", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

