// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($name:expr => $value:expr) => {
        println!("Name: {} =>  value: {}", $name, $value);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!("haj" => "på DAJ");
}
