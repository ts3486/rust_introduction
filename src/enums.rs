fn main() {
    enum IpAddrKind {
    V4,
    V6,
}

// enum with associated data

// assign data to each variant
enum IpAddr {
    V4(String),
    V6(String),
}

//
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn show_message(&self) {
        // method body would be defined here
        println!("Message called!");
    }
}
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));      
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    let m: Message = Message::Write(String::from("Hello"));
    m.show_message();

    // match function
    enum Color {
        Red,
        Green,
        Blue,
    }

    let c: Color = Color::Red;
    match c {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }
}   