#![allow(dead_code, unused_variables)]

mod basic;
mod closure_capture;
mod closure_as_parameter;
mod closure_as_return_value;


pub fn tests() {

    // basic::tests();

    // function_and_closure();

    // closure_capture::tests();

    // closure_as_parameter::tests();

    closure_as_return_value::tests();
}


fn function_and_closure() {
    // function and closure to implement add one to an u32.
    {
        fn add_one_1(x: u32) -> u32 { x + 1 }  // a function
        let add_one_2 = |x: u32| -> u32 { x + 1 };  // a full annotation closure
        let add_one_3 = |x| { x + 1 };  // a non-annotation closure, the annotation type will be deduced wen called.
        let add_one_4 = |x| x + 1;

        let u1 = add_one_1(10);
        let u2 = add_one_2(10);
        let u3 = add_one_3(10);
        let u4 = add_one_4(10);

        println!("{u1}, {u2}, {u3}, {u4}");  // 11, 11, 11, 11
    }

    // deduce annotation
    {
        let closure = |x| x;
        let s = closure(String::from("Hello"));
        println!("s:{s}");  // Hello

        let s2 = closure(String::from("Rust"));
        println!("s2:{s2}");  // Rust

        // When call closure multiple times, the parameter of each call must same.
        // If not, it's error.
        // let i = closure(1);  // Error: expected struct `String`, found integer.
    }
}

fn t() {
    {
        let x = 0;
        let c1 = |y| y == x;

        println!("{}", c1(1));  // false
        println!("{}", c1(0));  // true
    }
}