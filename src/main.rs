enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {

    }
}

fn main() {
    println!("Hello, world!");

    let m = Message::Write(String::from("Hello, World!"));
    m.call();
}