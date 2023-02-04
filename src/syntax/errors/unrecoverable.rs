#![allow(dead_code)]

pub fn errors() {
    call_panic();

    array_out_of_index();
}

fn call_panic() {
    println!("before call panic!");

    // panic!("call panic()");

    // println!("after call panic1");  // warning: unreachable statement
}

fn array_out_of_index() {
    let a = [1, 2, 3];
    println!("a[0]:{}", a[0]);

    // println!("a[3]:{}", a[3])

    let v = vec![1, 2, 3];
    println!("v[0]:{}", v[0]);
    println!("v[3]:{}", v[3]);
}