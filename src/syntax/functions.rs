#![allow(dead_code)]

pub fn basic_functions() {
    one_int_parameter(10);

    two_parameter(1, 10.1);

    let ret1 = return_int_1();
    let ret2 = return_int_2();
    let ret3 = return_int_3();
    let ret3_2 = return_int_3_2();
    println!("ret1:{ret1}, ret2:{ret2}, ret3:{ret3}, ret3_2:{ret3_2}");

    let plus1 = plus_one_1(10);
    let plus2 = plus_one_2(10);
    println!("plus1:{plus1}, plus2:{plus2}");

    let vals = two_parameter_and_return_two_int(10, 2);
    println!("vals:{:?}", vals);  // (12,8)
}

fn one_int_parameter(i :u32) {
    println!("one_int_parameter parameter i:{i}")
}

fn two_parameter(i: u32, f: f32) {
    println!("two_parameter parameter:{i},{f}");
}

fn return_int_1() -> i32 {
    return 1;
}
fn return_int_2() -> i32 {
    2
}
fn return_int_3() -> i32 {
    let x = 3;
    return x;
}
fn return_int_3_2() -> i32 {
    let x = 3;
    x
}

fn plus_one_1(x: i32) -> i32 {
    x + 1
}
fn plus_one_2(x: i32) -> i32 {
    return x + 1;
}

fn two_parameter_and_return_two_int(a:i32, b: i32) -> (i32, i32) {
    return (a + b, a-b);
}