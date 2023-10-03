use rust_oop::{Draw, Button, Screen, Post};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("Button")
            }),
        ]
    };

    let mut post = Post::new();

    post.add_text("This is a first draft of my blog post");
    println!("Post: {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("Post: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("Post: {}", post.content());
    assert_eq!("This is a first draft of my blog post", post.content());

}
