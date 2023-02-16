use std::mem::size_of_val;

pub fn tests() {
    let a = 1u8;
    let b = 2u16;
    let c = 3u64;
    let d = 4.0f32;
    // 1, 2, 8, 4
    println!("{}, {}, {}, {}", size_of_val(&a), size_of_val(&b), size_of_val(&c), size_of_val(&d));

    let x = 1;
    let y = 2;
    let z = 4.0;
    // 4, 4, 8
    println!("{}, {}, {}", size_of_val(&x), size_of_val(&y), size_of_val(&z));

}
