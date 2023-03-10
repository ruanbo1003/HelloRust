
pub fn tests() {
    // regular_reference();

    // use_box_like_reference();

    my_box_test();
}

fn regular_reference() {
    {
        let x = 10;
        let y = &x;

        println!("{}, {}", x, *y);

        assert_eq!(10, x);  // true
        assert_eq!(10, *y); // true
        // assert_eq!(10, y);  // error: can't compare `{integer}` with `&{integer}'
    }
}

fn use_box_like_reference() {
    {
        let x = 10;
        let y = Box::new(x);

        println!("{}, {}", x, *y);
        assert_eq!(10, x);  // true
        assert_eq!(10, *y); // true

        // assert_eq!(10, y); // error: can't compare `{integer}` with `Box<{integer}>`
    }
}

// implement MyBox: begin
// one element tuple struct.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn my_box_test() {
    {
        let x = 10;
        let y = MyBox(x);

        // error: type `MyBox<{integer}>` cannot be de-referenced.
        // implement std::ops::Deref for MyBox to solve this problem
        println!("{}, {}", x, *y);  // 10, 10
    }

    // deref coercions
    {
        let m = MyBox::new(String::from("Rust"));

        // deref from &string to &str.
        // this is valid, because the the String trait has a deref to &str.
        hello_str(&m);  // Hello, Rust!
    }

    // without deref coercions
    {
        let m = MyBox::new(String::from("Rust"));
        hello_str(&(*m)[..]);

        let s = &(*m)[..];
        println!("{}", s);  // Rust
    }
}
// implement MyBox: end

fn hello_str(name: &str) {
    println!("Hello, {}!", name);
}