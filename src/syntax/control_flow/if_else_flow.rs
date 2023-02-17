
pub fn tests() {
    // if - else
    {
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

    // if - else if - else
    {
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

    // if initialization
    {
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
}