use std::fmt::{Debug, Display};

pub fn tests() {
    empty_trait_bound();

    multiple_trait_bound();
}

fn empty_trait_bound() {
    struct Apple;
    struct Banana;
    struct Grape;

    trait Red {}
    trait Green {}

    impl Red for Apple {}
    impl Green for Grape {}

    fn red_item<T: Red>(_: &T) -> &'static str {
        "red"
    }
    fn green_item<T: Green>(_: &T) -> &'static str {
        "green"
    }

    let a = Apple;
    let _b = Banana;
    let g = Grape;

    println!("{}", red_item(&a));
    println!("{}", green_item(&g));

    // error: the trait bound `Red` is not satisfied.
    // println!("{}", red_item(&_b));  // error
}

fn multiple_trait_bound() {
    fn both_two_print<T: Display + Debug>(obj: &T) {
        println!("{}", obj);
        println!("{:?}", obj);
    }
    {
        let s = "hello";
        both_two_print(&s);
    }

    fn nor_print<T: Display, U: Debug>(obj1: &T, obj2: &U) {
        println!("{}", obj1);
        println!("{:?}", obj2);
    }
    {
        let i = 10;
        let a = [1, 2, 3];
        nor_print(&i, &a);
    }


}