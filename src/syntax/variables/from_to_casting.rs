use std::fmt::{Display, Formatter};

pub fn tests() {
    // from_into();

    // try_from_try_into();

    to_string();
}

fn from_into() {
    #[derive(Debug)]
    struct Number {
        val: i32,
    }

    // implement From trait for Number
    impl From<i32> for Number {
        fn from(data: i32) -> Self {
            Number { val: data }
        }
    }

    let num = Number::from(10);
    println!("{:?}", num);

    // if implemented th From trait, the Into trait
    // will automatically implemented by the Rust.
    let i = 100;
    let num: Number = i.into();
    println!("{:?}", num);  // Number { val: 100 }
}


fn try_from_try_into() {
    #[derive(Debug)]
    struct Number(i32);

    impl TryFrom<i32> for Number {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value >= 0 {
                Ok(Number(value))
            } else {
                Err(())
            }
        }
    }

    // try_from
    let n1 = Number::try_from(1);
    println!("{:?}", n1);  // Ok(Number(1))

    let n2 = Number::try_from(-1);
    println!("{:?}", n2);  // Err(())

    // try_into
    let res1: Result<Number, ()> = 8i32.try_into();
    println!("{:?}", res1);  // Ok(Number(8))

    let res2: Result<Number, ()> = (-1i32).try_into();
    println!("{:?}", res2); // Err(())
}


/*
implement `ToString` trait for the to_string() function.
but we'd better use the `std::fmt::Display` trait.
 */
fn to_string() {
    struct Circle {
        data: i32,
    }
    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle data:{}", self.data)
        }
    }

    struct Circle2 {
        data: i32,
    }
    impl Display for Circle2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle2 data(display):{}", self.data)
        }
    }

    let c1 = Circle{data: 1};
    println!("{}", c1.to_string());  // Circle data:1

    let c2 = Circle2{data: 2};
    println!("{}", c2);  // Circle2 data(display):2
}
