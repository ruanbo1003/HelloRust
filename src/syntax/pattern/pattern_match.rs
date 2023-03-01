
pub fn tests() {
    let i = 10;
    match i {
        1 => println!("one"),
        2 => {
            println!("two");
        },
        3 => {
            println!("three");
        },
        _ => {
            println!("others");
        },
    }
}