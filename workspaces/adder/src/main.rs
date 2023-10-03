use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );
}

// One cargo.lock file in the root of the directory is to ensure all packegs use the same versions of shared dependencies.
