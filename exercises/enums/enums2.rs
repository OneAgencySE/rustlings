// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(usize, usize, usize),
    Quit, //(Box<dyn Fn(usize)>),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        // Message::Quit(Box::new(|x| println!("Woop {}", x(5)))),
    ];

    for message in &messages {
        message.call();
    }
}
