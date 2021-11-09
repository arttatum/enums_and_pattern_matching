enum Message {
    Quit, // this variant is like the unit struct
    Move { x: i32, y: i32 }, // this variant has named fields
    Write(String), // this variant takes a single string value
    ChangeColor(i32, i32, i32), // this variant has tuple-like structure
}

// enums can have methods, yay!
impl Message {
    fn call(&self) {
        // method body would be defined here
        // Pattern matching here offers powerful functionality!
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}