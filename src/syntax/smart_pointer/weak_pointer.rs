use std::cell::RefCell;
use std::rc::Rc;

pub fn tests() {
    circular_reference_test();
}

struct Node {
    data: i32,
    next: RefCell<Rc<Option<Node>>>,
}

impl Node {
    fn set_next(&mut self, next: Rc<Option<Node>>) {
        *self.next.borrow_mut() = Rc::clone(&next);
    }

    fn new_node(data: i32) -> Rc<Option<Node>> {
        Rc::new(Some(Node {
            data,
            next: RefCell::new(Rc::new(None)),
        }))
    }
}

impl std::ops::Drop for Node {
    fn drop(&mut self) {
        println!("Node({}) drop", self.data)
    }
}

/*
   p1{ data:1, next:p2 }
   p2{ data:2, next:p1 }
 */

fn circular_reference_test() {
    {
        let p = Node::new_node(1001);
    }  // drop function of p will be called when out of scope

    {
        let p1 = Node::new_node(1002);
        let p2 = Node::new_node(1003);
    }  // drop functions of p1, p2 will be called when out of scope

    {
        let p1 = Node::new_node(1004);
        let p2 = Node::new_node(1005);
        let p3 = Node::new_node(1006);

        // p1.set_next(p2);
    } // drop functions of p1, p2, p3 will be called when out of scope

    {
        let p1 = Node::new_node(1);
        let p2 = Node::new_node(2);
        // p1.set_next(p2);
        // p2.set_next(p1);
    }
}
