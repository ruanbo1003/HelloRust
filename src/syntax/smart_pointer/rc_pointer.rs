
use std::rc::Rc;
use crate::syntax::smart_pointer::rc_pointer::List::{Cons, Nil};

/*
reference counting
can only be used in single thread.
 */
pub fn tests() {

    {
        let a = Rc::new(Cons(10, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("{}", Rc::strong_count(&a));  // 1

        let b = Cons(1, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));  // 2

        {
            let c = Cons(2, Rc::clone(&a));
            println!("{}", Rc::strong_count(&a)); // 3
        }

        println!("{}", Rc::strong_count(&a));  // 2
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
