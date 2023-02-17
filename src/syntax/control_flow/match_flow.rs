
pub fn tests() {
    {
        let num = 10;
        match num {
            1 => println!("num is 1"),
            2 | 3 | 4 => println!("num is in [2, 3, 4]"),
            5..=10 => {
                println!("num is in [5..10]");
            },
            _ => {
                println!("other num value");
            }
        }
    }

    // match expression
    {
        let b = true;
        let binary = match b {
            false => 0,
            true => 1,
        };
        println!("{}", b);  // 1
    }

    // match_with_tuple();

    // match_with_enum();

    // match_with_if();

    match_bind();
}

fn match_with_tuple() {
    {
        let triple = (0, 1, 2);
        match triple {
            (0, a, b) => println!("first is 0, other not known"),
            (1, 1, a) => println!("first two is 0, 1, other not known"),
            (_, _, _) => {}
        }
    }

    {
        let triple = (1, 2, 3);
        match triple {
            (0, a, b) => {
                println!("first is 0, other not known");
            },
            (1, 2, a) => {
                println!("first two is 1, 2");
            },
            (_, _, _) => {
                println!("don't care the values");
            }
        }
    }
}

fn match_with_enum() {
    enum Color {
        Red,
        Green(i32),
        Yellow(String),
        Gray(i32, i32, i32),
    }

    let c1 = Color::Red;
    let color_tuples = vec![
        Color::Red,
        Color::Green(1),
        Color::Yellow(String::from("tree")),
        Color::Gray(10, 20, 30),
    ];

    for each in color_tuples.iter() {
        match each {
            Color::Red => println!("red"),
            Color::Green(i) => {
                println!("green:{}", i);
            },
            Color::Yellow(s) => {
                println!("yellow:{}", s);
            },
            Color::Gray(x, y, z) => {
                println!("gray({}, {}, {})", x, y, z);
            }
        }
    }
}

fn match_with_if() {
    let pairs = vec![
        (0, 0),  // x == y
        (0, 1),  // x == 0
        (1, 0),  // y == 0
        (10, 100),  // don't care
    ];

    for each_pair in pairs.iter() {
        match *each_pair {
            (x, y) if x == y => {
                println!("x == y");
            },
            (x, y) if x == 0 => {
                println!("x == 0");
            },
            (x, y) if y == 0 => {
                println!("y == 0");
            },
            _ => {
                println!("don't care");
            }
        }
    }
}

fn match_bind() {
    {
        fn age() -> i32 {
            10
        }

        match age() {
            0 => {
                println!("age is zero");
            },
            n @ 1..=10 => {
                println!("age is {}", n);
            },
            n @ 11..=100 => {
                println!("{}", n);
            },
            _ => {
                println!("don't care");
            }
        }
    }

    // use bind to destructure enum
    {
        fn number() -> Option<i32> {
            Some(11)
        }

        match number() {
            Some(n @ 10) => {
                println!("number is {}", 10);
            },
            Some(n) => {
                println!("{}", n);
            },
            _ => (),
        }
    }

}