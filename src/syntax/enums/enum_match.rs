#![allow(dead_code)]

pub fn tests() {
    plus_one_test();

    catch_all_pattern();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky Dime");
            10
        }
        Coin::Quarter => 25,
    }
}

fn option_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one_test() {
    let five = Some(5);

    // Some(5) match Some(i)
    let r = option_plus_one(five);
    println!("r:{:?}", r);  // Some(6)

    let none = option_plus_one(None);
    println!("none:{:?}", none);  // None
}

fn catch_all_pattern() {
    let val: u8 = 10;
    match val {
        1 => {
            println!("one")
        },
        2 => {
            println!("two")
        },
        3 => {
            println!("three")
        },
        _ => {
            println!("bigger then three")
        }
    }
}