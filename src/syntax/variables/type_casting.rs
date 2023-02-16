
/*
use as to cast one type to another.
 */

pub fn tests() {
    // decimal to integer
    {
        let d = 65.1122_f32;

        // expected `u8`, found `f32`
        // let i1: u8 = d; // error

        let i2: u8 = d as u8;  // cast f32 to u8
        let c: char = i2 as char;  // cast u8 to char

        println!("{}, {}", d, i2);  // 65.1122, 65
        println!("{}", c);  // A
    }
}
