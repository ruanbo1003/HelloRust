
pub fn tests() {
    let my_color: Option<&str> = None;
    let is_friday = false;
    let age: Result<u8, _> = "20".parse();

    if let Some(color) = my_color {
        println!("my color");
    } else if is_friday {
        println!("friday color");
    } else if let Ok(age) = age {
        if age > 20 {
            println!("bigger then 20");
        } else {
            println!("young color");
        }
    }
}