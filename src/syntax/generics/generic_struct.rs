
pub fn tests() {
    // declare generic struct instance.
    // generic has only one type to be specified
    {
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:2.2, y:2.2};

        println!("{:?}", p1);  // Point { x: 1, y: 1 }
        println!("{:?}", p2);  // Point { x: 2.2, y: 2.2 }

        // ERROR: `Point` generic can only has one type, but `x` and `y` have two types here.
        // let p3 = Point{x:1, y:1.1};  // error: expected `i32`, found `f64`
    }

    // generic struct function for all instance with all types
    {
        let p1 = Point{x:1, y:1};
        let p2 = Point {x:2.2, y:2.2};
        let p3 = Point {x:'a', y:'a' };

        p1.foo();  // Point::foo
        p2.foo();  // Point::foo
        p3.foo();  // Point::foo
    }

    // generic struct function for specified instance type: i32
    {
        // the generic type of p1 is: i32
        let p1 = Point{x:1, y:1};
        p1.bar();  // Point::bar
        p1.i32_only();

        p1.foo();  // can call `foo` too.

        // the generic type of _p2 is: f64
        let p2 = Point{x:2.2, y:2.2};
        // error: no method named `bar` for struct `generic_struct::Point<{float}>` int the
        // current scope.
        // p2.bar();  // error:
        p2.f64_only();  // f64_only
    }

    // generic struct of two type
    {
        let two_type_point = Point2{x:1, y:2.2};
        println!("two type point: {:?}", two_type_point);
    }

}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn foo(&self) {
        println!("Point::foo")
    }
}

impl Point<i32> {
    fn i32_only(&self) {
        println!("i32_only")
    }

    fn bar(&self) {
        println!("Point::bar")
    }
}

impl Point<f64> {
    fn f64_only(&self) {
        println!("f64_only");
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
