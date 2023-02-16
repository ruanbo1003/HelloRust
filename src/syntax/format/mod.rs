#![allow(dead_code, unused_variables)]


use std::fmt;
use std::fmt::{Formatter};

pub fn tests() {
    // format_output();

    // debug_output();

    display_output();
}

fn format_output() {
    // basic format
    {
        println!("{}", "hello");

        println!("{} {}", "hello", "world");

        let h = "hello";
        let w = "world";
        println!("{} {}", h, w);
    }

    // index format
    {
        println!("{0} {1}", "hello", "world");

        println!("{0} {1}, {0} {1}", "hello", "world");

        let h = "hello";
        let w = "world";
        println!("{0} {1}, {0} {1}", h, w);
    }

    // parameter format
    {
        println!("{h} {w}, {h} {w}", h="hello", w="world");
    }
}

/*
Debug
#[derive(Debug)]
 */
fn debug_output() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct Printable(i32);

    #[derive(Debug)]
    struct Deep(Printable);

    {
        let s1 = UnPrintable(1);

        // error: `UnPrintable` doesn't implement `Debug`
        // println!("{:?}", s1);  // error

        let s2 = Printable(2);
        println!("{:?}", s2);  // Printable(2)

        let d = Deep(Printable(3));
        println!("{:?}", d);  // Deep(Printable(3))
        println!("{:#?}", d);  // more beautiful output
    }
}


/*
std::fmt::Display
 */
fn display_output() {

    struct Temp(i32);
    impl fmt::Display for Temp {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Temp({})", self.0)
        }
    }

    {
        let t = Temp(1);
        println!("{}", t);  // Temp(1)
    }

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (idx, val) in vec.iter().enumerate() {
                if idx != 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{}", val)?;
            }

            write!(f, "]")
        }
    }

    {
        let l1 = List(vec![]);
        println!("{}", l1);  // []

        let l2 = List(vec![1, 2, 3]);
        println!("{}", l2);  // [1, 2, 3]
    }
}