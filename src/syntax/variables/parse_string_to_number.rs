
/*
parse string to number.
 */
pub fn tests() {
    // parse
    {
        let i:i32 = "5".parse().unwrap();
        println!("{}", i);  // 5

        let e: i32 = "a".parse().unwrap_or(0);
        println!("{}", e);  // 0
    }

    // turbo fish
    {
        let i = "10".parse::<i32>().unwrap();
        println!("{}", i);  // 10

        let e = "ab".parse::<i32>().unwrap_or(0);
        println!("{}", e);
    }
}

