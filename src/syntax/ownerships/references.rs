#![allow(dead_code)]

pub fn tests() {
    // pass reference(&s1) rather than value(s1)
    {
        let s1 = String::from("hello");
        let str_len = cal_string_len(&s1);  // cal_string_len() function will not take ownership of s1
        println!("after cal_string_len, s1:{s1}, len:{str_len}"); // hello, 5
    }

    // reference and update reference value
    {
        let mut s = String::from("hello");
        immutable_reference_and_update(&s);

        mutable_reference_and_update(&mut s);
        println!("after mutable reference and update:{s}"); // hello, world
    }

    // we can create two immutable reference of s
    {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("two immutable reference, r1:{r1}, r2:{r2}"); // hello, hello
    }

    // can we create two mutable reference?
    {
        // create one, use one; then create another, use another: valid
        {
            let mut s = String::from("hello");
            let r1 = &mut s;
            println!("one mutable reference, r1:{r1}");  // hello

            let r2 = &mut s;
            println!("r2:{r2}");
        }

        // create one, create another, do not use: valid
        {
            let mut s = String::from("hello");
            let _r1 = &mut s;
            let _r2 = &mut s;
        }

        // create one, create another, and use the first references: invalid
        {
            let mut s = String::from("hello");
            let _r1 = &mut s;
            let _r2 = &mut s;  // error: cannot borrow `s` as mutable more than once at a time

            // println!("_r1:{_r1}");
        }

        // create one, create another, and use the second references: valid
        {
            let mut s = String::from("hello");
            let _r1 = &mut s;
            let _r2 = &mut s;

            println!("_r2:{_r2}");
        }

        // create one, create another, and use the two references: invalid
        {
            let mut s = String::from("hello");
            let _r1 = &mut s;
            let _r2 = &mut s;

            // println!("_r1:{_r1}, _r2:{_r2}"); // error: cannot borrow `s` as mutable more than once at a time
        }
    }

    println!("dangling references");
    // let reference_to_nothing = dangling_reference();
}

fn cal_string_len(src: &String) -> usize { // src is a reference to a String
    return src.len();
}  // Here, src goes out of scope. But src is a reference and it does not have ownership of the original string,
// it does not drop


// immutable reference
fn immutable_reference_and_update(_src: &String) {
    // Cannot borrow immutable local variable `src` as mutable
    // src.push_str(", world");  // error

}

fn mutable_reference_and_update(src: &mut String) {
    src.push_str(", world");
}

// error: missing lifetime specifier
// fn dangling_reference() -> &String {  // error
//     let s = String::from("hello");
//     return &s;
// }
