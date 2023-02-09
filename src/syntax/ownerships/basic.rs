#![allow(dead_code)]

pub fn ownership_tests() {
    // first_test_with_string();

    // move_1();

    // ownership_function();

    return_value();

}

fn first_test_with_string() {
    {
        let s = String::from("hello");  // s is valid from this point
        println!("s:{s}")
    }  // out of scope, s is invalid from now on. Rust will call drop automatically at the closing curly bracket.
}

fn move_1() {
    // integer move
    {
        let x = 10;
        println!("x:{x}");  // 10

        let y = x;
        println!("y:{y}");  // 10

        println!("x:{x}");  // 10
    }

    // string move
    {
        let s1 = String::from("hello");
        println!("s1:{s1}");  // hello

        // After this let s2 = s1; Rust considers s1 as no longer valid.
        // Therefore, Rust doesn't need to free anything when s1 goes out of scope.
        let s2 = s1;
        println!("s2:{s2}");  // hello

        // println!("s1:{s1}");  // error: borrow of moved value: `s1`
        // println!("s1.len():{}, s1.capacity():{}", s1.len(), s1.capacity()); // error: borrow of moved value: `s1`
    }
}

fn ownership_function() {
    // String
    {
        let s = String::from("hello");
        take_ownership(s);

        // error: borrow of moved value: `s`
        // println!("after task_ownership, s:{s}");  // error
    }

    // integer, i32
    {
        let a: i32 = 100;
        make_copy(a);

        println!("after make_copy, a:{a}");  // 100
    }
}
fn take_ownership(str: String) {
    println!("take_ownership,str:{str}");
}

fn make_copy(val: i32) {
    println!("make_copy, val:{val}");
}

/*
 Return value can transfer ownership.
 */
fn return_value() {
    {
        let s = return_value_and_give_ownership();
        println!("get ownership from return value, s:{s}");  // hello-return
    }

    {
        let s1 = String::from("hello-take-and-back");
        println!("s1:{s1}");

        let s2 = take_ownership_and_give_back(s1);
        println!("s2:{s2}");
    }
}
fn return_value_and_give_ownership() -> String {
    let s = String::from("hello-return");
    println!("s in function:{s}");  // hello-return
    return s;
}
fn take_ownership_and_give_back(src: String) -> String {
    println!("src in function:{src}");
    return src;
}