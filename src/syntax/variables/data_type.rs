
pub fn tests() {
    // Scalar Types
    {
        data_types_integer();

        data_types_floating_point();

        data_types_boolean();

        data_types_character();
    }

    // compound types
    {
        data_types_tuple();

        data_types_array();
    }
}

fn data_types_integer() {
    // integer types
    let i1: i8 = 8;  // signed integer 8-bit
    let ui1: u8 = 8; // unsigned integer 8-bit
    println!("i1: {i1}, ui1:{ui1}");

    let i2 : i16 = 16;  // signed integer 16-bit
    let ui2 : u16 = 16; // unsigned integer 16-bit
    println!("i2:{i2}, ui2:{ui2}");

    let i3 : i32 = 32;  // signed integer 32-bit
    let ui3 : u32 = 32; // unsigned integer 32-bit
    println!("i3:{i3}, ui3:{ui3}");

    let i4 : i64 = 64;  // signed integer 64-bit
    let ui4: u64 = 64;  // unsigned integer 64-bit
    println!("i4:{i4}, ui4{ui4}");

    let i5: i128 = 128;  // signed integer 128-bit
    let ui5 :u128 = 128;  // unsigned integer 128-bit
    println!("i5:{i5}, ui5:{ui5}");

    let i6: isize = 0; // signed size
    let ui6 : usize = 0; // unsigned size
    println!("i6:{i6}, ui6:{ui6}");

    // number operations
    let sum = 1 + 2;  // addition
    let sub = 10.1 - 1.0;  // subtraction
    let product = 11 * 3;  // multiplication
    println!("sum:{sum}, sub:{sub}, product:{product}"); // 3, 9.1, 33

    let div1 = 4 / 5;  // division
    let div2 = 4.0 / 5.0;  // division
    println!("div1:{div1}, div2:{div2}");  // 0, 0.8

    let reminder = 11 % 5;  // reminder
    println!("reminder:{reminder}")  // 1
}

fn data_types_floating_point() {
    // floating-point types
    let f1 = 10.0;
    let f2: f32 = 1.1;

    println!("f1:{f1}, f2:{f2}");
}

fn data_types_boolean() {
    // boolean type
    let b1 = true;
    println!("b1:{b1}");  // true

    let b2 = false;
    println!("b2:{b2}");  // false

    let b3: bool = true;  // with explicit type annotation
    println!("b3:{b3}");
}

fn data_types_character() {
    // the character type
    let c1 = 'a';
    println!("c1:{c1}");  // a

    let c2 : char = 'b';  // declare a character type with explicit type annotation
    println!("c2:{c2}");
}

fn data_types_tuple() {
    // the tuple type

    let t1 = (1, 1.0, 'a');
    println!("t1.0:{}, t1.1:{}, t1.2:{}", t1.0, t1.1, t1.2);

    let t2:(i8, f32, char) = (1, 10.1, 'b');
    println!("t2.0:{}, t2.1:{}, t2.2:{}", t2.0, t2.1, t2.2);

    let (x, y, z) = t2;
    println!("x:{x}, y:{y}, z:{z}");

    let one = t2.0;
    let two = t2.1;
    let three = t2.2;
    println!("one:{one}, two:{two}, three:{three}");
}

fn data_types_array() {
    // the array type

    let a1 = [1, 2, 3, 4, 5];
    println!("a1:{:?}", a1);  // [1, 2, 3, 4, 5]

    let a2:[i8; 3] = [1, 2, 3];
    println!("a2:{:?}", a2);  // [1, 2, 3]

    // let a3:[i8; 3] = [1, 2];  // error: mismatched types, expected `[i8;3]`, found `[i8;2]`

    let months = ["m1", "m2", "m3"];
    println!("months:{:?}", months);

    // let vals = [1, "m1"];  // error: expected integer, found '&str'

    // access array elements
    let first = a1[0];
    let last = a1[a1.len()-1];
    println!("first:{first}, last:{last}")  // 1, 5
}