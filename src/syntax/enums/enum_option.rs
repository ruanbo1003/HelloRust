#![allow(dead_code)]

pub fn tests() {
    option_some();

}

fn option_some() {
    let some_number = Some(10);
    println!("some_number:{:?}", some_number);  // Some(10)

    let some_char = Some('a');
    println!("some_char:{:?}", some_char);  // Some('a')

    let absent_number: Option<i32> = None;
    println!("absent_number:{:?}", absent_number);  // None
}