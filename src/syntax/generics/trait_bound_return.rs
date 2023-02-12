#![allow(unused_variables)]

pub fn tests() {
    {
        let w1 = return_one_trait_person();
        w1.go();  // Person::go

        let w2 = return_one_trait_dog();
        w2.go();  // Dog::go
    }
}

// test function: return a variable that implemented the Walk trait.
fn return_one_trait_person() -> impl Walk {
    return Person{};
}
fn return_one_trait_dog() -> impl Walk {
    return Dog{};
}

fn multiple_return_expression_one_type(data: i32) -> impl Walk {
    if data > 10 {
        return Person{};
    } else {
        return Person{};
    }
}

// error: we cannot return different types of trait implemented struct in different `return expression`
// expected struct `Person`, found struct `Dog`
// fn multiple_return_expression_two_types(data: i32) -> impl Walk {
//     if data > 10 {
//         return Person{}
//     } else {
//         return Dog{};
//     }
// }


// declare one trait Walk and two struct Person and Dog.
trait Walk {
    fn go(&self);
}
struct Person {
}
impl Walk for Person {
    fn go(&self) {
        println!("Person::go");
    }
}

struct Dog {
}
impl Walk for Dog {
    fn go(&self) {
        println!("Dog::go");
    }
}