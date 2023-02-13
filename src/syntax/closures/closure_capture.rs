#![allow(dead_code, unused_variables)]

pub fn tests() {
    // closure_capture_example();

    three_kinds_of_captures();

    // force_move_capture();
}

fn closure_capture_example() {
    // closure will capture the variables where it been declared.
    {
        let x = 4;

        // x in not a parameter of closure `equal_to_x`,
        // we can still use x in the body of closure `equal_to_x`
        let equal_to_x = |z| z == x;
        let y = 4;
        println!("is equal to:{}", equal_to_x(y));  // true
    }

    // function can not capture the variables.
    {
        let x = 4;

        // error: can't capture dynamic environment in a fn item.
        // fn equal_to_x(z: u32) -> bool { z == x }
    }
}

fn three_kinds_of_captures() {
    // FnOnce: `once` means the closure cannot capture the environment's variable ownership more
    // than once. can only be call once.
    {

    }

    // FnMut: not-move capture, can change the environment's variable.
    {
        let mut x = 1;

        // closure `equal_to_x` captured the environment's variable x,
        // and only read the x.
        let mut equal_to_x = |z|{ x=10; z == x};

        let a = 1;
        println!("a is equal to x:{}", equal_to_x(a));  // false

        let b = 10;
        println!("b is equal to x:{}", equal_to_x(b));  // true

        println!("{}", x);  // 10
    }

    // Fn: capture the environment's variable, and can't change the value.
    {
        let x = 1;
        let equal_to_x = |y| y == x;

        let a = 1;
        println!("{}", equal_to_x(a));  // true
    }
}

fn force_move_capture() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // error: borrow of moved value: `x`
    // println!("x after closure:{:?}", x);

    let y = vec![1, 2, 3];

    println!("x == y:{}", equal_to_x(y));
}