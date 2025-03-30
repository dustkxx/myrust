#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    // Tuple struct
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method
        println!("Message is : {:#?}", self);
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move { x: 10, y: 20 };
    m.call();
    let m = Message::ChangeColor(255, 0, 0);
    m.call();
    let quit = Message::Quit;
    quit.call();
}