#![allow(dead_code)]

pub fn basic_control_flow() {
    // if expression
    {
        // if_expression();

        // if_and_else_if_expression();

        // if_in_let();
    }

    // loop repetition
    {
        // repetition_loop();

        // repetition_while();

        visit_collection_with_for();
    }
}

fn if_expression() {
    let x = 10;
    if x < 10 {
        println!("x < 10")
    } else {
        println!("x > 10")
    }

    // error: expected `bool`, found integer
    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    // We must explicit provide a Boolean as its condition
    // if x {
    //     println!("x if true")
    // }
}

fn if_and_else_if_expression() {
    let number = 2;

    if number > 10 {
        println!("number is greater then 10")
    } else if number > 5 {
        println!("number is greater then 5")
    } else if number > 2 {
        println!("number is greater then 2")
    } else {
        println!("number is less or equal then 2")
    }
}

fn if_in_let() {
    let cond1 = true;
    let num1 = if cond1 { 1 } else { 2 };
    println!("if_in_let: num1:{num1}");  // 1

    let cond2 = false;
    let num2 = if cond2 { 1 } else { 2 };
    println!("if_in_let, num2:{num2}");  // 2

    // the value type of if and else must be the same
    // let cond3 = true;
    // let val3 = if cond3 { 1 } else { "hello" };  // error: `if` and `else` have incompatible types.
}

fn repetition_loop() {
    let mut counter = 1;
    loop {
        if counter > 3 {
            break;  // break the loop
        }
        println!("loop {counter}");
        counter += 1;
    }

    // return value from loop
    let mut counter2 = 0;
    let result2 = loop {
        counter2 += 1;
        if counter2 >= 10 {
            break counter2 * 2;  // break the loop and return a value
        }
    };
    println!("loop value result2:{result2}"); // 20

    let mut counter3 = 0;
    'outer_loop: loop {
        println!("counter3:{counter3}");

        let mut inner_counter = 0;
        loop {
            inner_counter += 1;
            println!("inner_counter:{inner_counter}");
            if inner_counter == 2 {
                break;
            }

            if counter3 == 3 {
                break 'outer_loop;  // break the specified loop
            }
        }

        counter3 += 1;
    }
    println!("nested loop, counter3:{counter3}");  // 3
}

fn repetition_while() {
    let mut num1 = 3;
    while num1 != 0 {
        println!("num1:{num1}");  // 3, 2, 1
        num1 -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while idx < 5 {
        println!("the value of a[{idx}] is {}", a[idx]);  // 1, 2, 3, 4, 5
        idx += 1;
    }
}

fn visit_collection_with_for() {
    let a1 = [1, 2, 3, 4, 5];
    for element in a1 {
        println!("the value is: {element}");  // 1, 2, 3, 4, 5
    }

    for number in 1..4 {
        println!("the number-1 is {number}");  // 1, 2, 3
    }
    for number in (1..4).rev() {
        println!("the number-2 is {number}");  // 3, 2, 1
    }
}