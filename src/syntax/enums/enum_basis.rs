#![allow(dead_code)]


pub fn tests() {
    // define_and_declare();

    // many_types_enum();

    c_style_enum();
}

enum IpAddrKind {
    V4,
    V6,
}


#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn define_and_declare() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home:{:?}, loopback:{:?}", home, loopback);
}


#[derive(Debug)]
enum Message {
    Quit,  // no data associated
    Move { x: i32, y: i32},  // named fields like struct
    Write(String),  // include a single String
    ChangeColor(i32, i32, i32),  // three i32 values
}
impl Message {
    fn call(&self) {
        println!("{:?} message call", self);
    }
}

struct QuitMessage;  // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);  // tuple struct
struct ChangeColorMessage(i32, i32, i32);  // tuple struct

fn many_types_enum() {
    let q = Message::Quit;
    q.call();

    let m = Message::Move {x:10, y:20};
    m.call();

    let w = Message::Write(String::from("hello"));
    w.call();

    let c = Message::ChangeColor(1, 2, 3);
    c.call();
}

// C style enum
#[derive(Debug)]
enum Number {
    Red = 0,
    Green = 1,
    Yellow = 2,
}
fn c_style_enum() {
    let n1 = Number::Red;
    println!("{:?}", n1);  // Red

    let n2 = Number::Green;
    println!("{}", n2 as i32);
}
