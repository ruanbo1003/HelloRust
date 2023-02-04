#![allow(dead_code)]
#![allow(unused_variables)]

pub fn variables() {
    // default variable if immutable
    let v1 = 1;
    println!("value of v1 is {v1}");
    // v1 = 2;  // error, Cannot assign twice to immutable variable

    // define variable as mut, so we can update the variable value
    let mut v2 = 1;
    println!("value of v2 is {v2}");
    v2 = 10;
    println!("value of v2 is {v2}");

    // can declare a new variable with the same name before.
    // say: the previous v2 is shadowed by the second v2.
    // the second v2 is over-shadow the first v2 variable.
    let v2 = 100;
    println!("the new value of v2 is {v2}");  // 100
    // v2 = 101; // error: Cannot assign twice to immutable variable

    {
        let v2 = 105;
        println!("the v2 value in brace is {v2}");  // 105
    }
    println!("the v2 value out of brace is {v2}");  // 100

    // const is immutable, can not be signed by mut, can not update at any situation
    // the value of the const must be constant expression
    const ONE_HOUR_SECONDS : u32 = 1 * 60 * 60;
    println!("one hour seconds:{ONE_HOUR_SECONDS}");

    // we can declare two different data type variable with the same variable name.
    let str1 = r"hello";
    println!("value of first str1:{str1}");  // hello
    let str1 = str1.len();
    println!("value of second str1:{str1}");  // 5

    // but when we declare a mutable variable.
    // we cannot update the value with a different data type.
    let mut str_len = 10;
    // str_len = "hello";  // mismatched types.
    println!("value of str_len:{str_len}");
    str_len = 11;
    println!("new value of str_len:{str_len}")
}