// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

struct Celsius(i32);
struct Fahrenheit(i32);
struct Kelvin(u32);

enum Degrees {
    K(Kelvin),
    F(Fahrenheit),
    C(Celsius),
}

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor, // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
