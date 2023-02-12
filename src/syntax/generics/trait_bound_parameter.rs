#![allow(unused_variables)]

pub fn tests() {
    // parameter_is_one_trait_type_tests();

    parameter_is_multiple_trait_implemented_tests();
}

// ==== test: one/multiple trait as function parameter ====
fn parameter_is_one_trait_type_tests() {
    // function receive one trait type argument.
    // all struct the implemented the trait can pass to the function.
    {
        let p = Person{};
        one_trait_parameter_func(p);  // Person::go

        let d = Dog{};
        one_trait_parameter_func(d);  // Dog::go
    }

    // pass two struct instance pass to function that take two straits parameter.
    {
        // we can pass two Person type to the function.
        {
            let p1 = Person{};
            let p2 = Person{};
            two_trait_parameter_func(p1, p2);  // Person::go, Person::go
        }

        // we can pass a Person and a Dog to the function.
        {
            let p = Person{};
            let d = Dog{};
            two_trait_parameter_func(p, d);  // Person::go, Dog::go
        }

        // we can also pass a Dog and a Person to the function.
        {
            let p = Person{};
            let d = Dog{};
            two_trait_parameter_func(d, p);  // Dog::go, Person::go
        }
    }

    // the function's parameter is declared by template traits.
    {
        // pass two Person to the function
        {
            let p1 = Person{};
            let p2 = Person{};
            two_trait_parameter_func_with_template(p1, p2);  // Person::go Person::go
        }

        // pass two Dog to the person
        {
            let d1 = Dog{};
            let d2 = Dog{};
            two_trait_parameter_func_with_template(d1, d2);  // Dog::go Dog::go
        }

        // but we can pass a Person and a Dog (different trait implement type) to the function.
        {
            let p = Person{};
            let d = Dog{};
            // error: expected `Person`, found `Dog`
            // two_trait_parameter_func_with_template(p, d); // error
        }
    }
}

// test: function is multiple strait implemented type
fn parameter_is_multiple_trait_implemented_tests() {
    {
        let d = Dog{};
        one_parameter_with_multiple_trait_type_1(d);  // Dog::go wang-wang

        let p = Person{};
        one_parameter_with_multiple_trait_type_1(p);  // Person::go ha-ha
    }

    {
        let d = Dog{};
        one_parameter_with_multiple_trait_type_2(d);  // Dog::go wang-wang
    }

    {
        let d = Dog{};
        one_parameter_with_multiple_trait_type_3(d);  // Dog::go wang-wang
    }

    // only the parameter the implemented both Walk and Speak can pass to the function.
    {
        // let c = Cat{};
        // one_parameter_with_multiple_trait_type_1(c);
    }
}

// declare a function that receive one trait argument.
fn one_trait_parameter_func(walk: impl Walk) {
    walk.go();
}

// declare a function that receive two trait parameters.
fn two_trait_parameter_func(walk1: impl Walk, walk2: impl Walk) {
    walk1.go();
    walk2.go();
}

// use template to specify the trait type for multiple parameters
fn two_trait_parameter_func_with_template<T: Walk>(walk1 :T, walk2: T) {
    walk1.go();
    walk2.go();
}

// one parameter, but can be multiple trait types
fn one_parameter_with_multiple_trait_type_1(item: impl Walk + Speak) {
    item.go();
    item.hello();
}
fn one_parameter_with_multiple_trait_type_2<T: Walk + Speak>(item: T) {
    item.go();
    item.hello();
}
fn one_parameter_with_multiple_trait_type_3<T>(item: T)
    where T: Walk + Speak
{
    item.go();
    item.hello();
}


// ==== declare two traits and three struct ====
// two traits, Walk and Speak
trait Walk {
    fn go(&self) {
        println!("Walk::go")
    }
}

trait Speak {
    fn hello(&self);
}

// Person struct and trait implement for Person
struct Person {
}

impl Walk for Person {
    fn go(&self) {
        println!("Person::go");
    }
}

impl Speak for Person {
    fn hello(&self) {
        println!("ha-ha")
    }
}

// struct Dog and traits implement for Dog
struct Dog {
}

impl Walk for Dog {
    fn go(&self) {
        println!("Dog::go");
    }
}

impl Speak for Dog {
    fn hello(&self) {
        println!("wang-wang");
    }
}

// struct Cat and cat only implement Speak trait
struct Cat {
}

impl Speak for Cat {
    fn hello(&self) {
        println!("mio-mio")
    }
}
