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
    #[derive(Debug)]
    struct Capture {
        data: i32,
    }

    // `&T` capture: immutable reference capture
    {
        let c = Capture { data: 1 };
        let print = || { println!("{:?}", c); };
        print();  // Capture { data: 1 }

        let _borrow = &c;  // immutable reference here

        // after immutable reference, print() is still valid.
        print();  // Capture { data: 1 }
    }

    // `&mut T` capture: mutable reference capture
    {
        let mut count = 0;
        let mut inc = || {
            count += 1;
            println!("count:{}", count);
        };

        inc();  // count:1

        // error
        // let mut _borrow = &count;

        inc();
    }

    // `T` capture: value capture
    {
        let b = Box::new(1);

        // closure cannot be invoked more than once because it moves the variable `b` out of its environment.
        let consume = || {
            println!("b: {:?}", b);
            std::mem::drop(b);
        };

        // this value implements `FnOnce`, which causes it to be moved when called.
        consume();  // b: 1

        // error: use of moved value: `consume`
        // consume();  // error
    }
}

// add `move` before `||` to force move.
fn force_move_capture() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // error: borrow of moved value: `x`
    // println!("x after closure:{:?}", x);

    let y = vec![1, 2, 3];
    println!("x == y:{}", equal_to_x(y));

    {
        #[derive(Debug)]
        struct ForceMove {
            data: i32,
        }

        // normal move, fm is valid after print_fm();
        {
            let fm = ForceMove{ data: 1 };
            let print_fm = || { println!("{:?}", fm); };
            print_fm();  // ForceMove { data: 1 }
            println!("{:?}", fm);  // ForceMove { data: 1 }
        }

        // force move, fm is not valid after print_fm();
        {
            let fm = ForceMove { data: 1 };
            let print_fm = move || { println!("{:?}", fm); };
            print_fm();

            // error: borrowed of moved value: `fm`
            // println!("{:?}", fm);  // error
        }
    }
}