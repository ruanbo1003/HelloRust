#![allow(dead_code, unused_variables)]

mod lifetime_in_function_call;

pub fn tests() {
    // dangling_reference();

    lifetime_in_function_call::tests();
}


/*
an example where dangling reference comes.
 */
fn dangling_reference() {
    let r: i32 = 0;

    // inner scope
    {
        let x = 5;

        // error: `x` does not live long enough.
        // r = &x; //borrowed value does not live long enough.
    }  // `x` dropped here wile still borrowed.

    println!("r:{}", r);
}