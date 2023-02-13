#![allow(unused_assignments)]

pub fn tests() {
    // use lifetime-annotation
    {
        let s1 = String::from("HelloRust");
        let s2 = "world";
        let longest_s = longest_str_2(s1.as_str(), s2);

        println!("longest str:{}", longest_s);  // HelloRust

        println!("s1:{}", s1);  // HelloRust
        println!("s2:{}", s2);  // world
    }

    lifetime_cases();
}

/*
the function `longest_str_1` report an error and compile time.
fn longest_str_1(x: &str, x: str) -> &str {
                                     ^ expected name lifetime parameter.
this function's return type contains a borrowed value, but the signature does not say
it is borrowed from `x` or `x`. consider introducing a named lifetime parameter.

When we declare this `longest_str_1` function:
    1. we don't know what argument will be passed to it.
    2. we don't know which branch will be executed, `if` or `else`.
    3. we don't know the lifetime of the two reference argument, `x` and `x`.
So we cannot know if the returned reference is always valid, by compare the scope of the
input reference argument.
The borrow-checker don't know either, because it don't know how the lifetime
of `x` and `x` associated with the lifetime of returned value.
To fix this, we need to use the lifetime-annotation, `longest_str_2`

 */
// fn longest_str_1(x: &str, y: &str) -> &str {
//     return if a.len() >= b.len() {
//         x
//     } else {
//         y
//     }
// }

/*
use the lifetime-annotation
 */
fn longest_str_2<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/*

 */
fn lifetime_cases() {
    // scope of `s1`, `s2` and `longest` are the same.
    {
        let s1 = "HelloWorld";
        let s2 = "Rust";
        let longest = longest_str_2(s1, s2);

        println!("longest:{}", longest);  // HelloWorld
        println!("{}, {}", s1, s2); // HelloWorld, Rust
    }

    // scope of `s1` is smaller then `s2`
    // scope of `longest` is the same as `s2`
    {
        // OK case: use `longest` in scope of `s2`
        {
            let s1 = String::from("HelloWorld");
            {
                let s2 = String::from("Rust");
                let longest = longest_str_2(s1.as_str(), s2.as_str());

                println!("longest:{}", longest); // HelloWorld
            }
        }

        // Error case: use `longest` out scope of `s2`, in scope `s1`
        {
            let s1 = String::from("HelloWorld");
            let longest;
            {
                let s2 = String::from("Rust");
                longest = longest_str_2(s1.as_str(), s2.as_str());
            } // `s2` dropped here while still borrowed.

            // error: `s2` does not live long enough.
            // Notice that, the error info says `s2`, not `s1`, even though the `longest` is `s1`
            // in this case.
            // println!("longest:{}", longest);  // borrowed later used here.
        }

    }

    {

    }
}