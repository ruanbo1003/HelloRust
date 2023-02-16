
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn push_head(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn display(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.display())
            },
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn tests() {
    let mut list = List::new();

    list = list.push_head(1);
    list = list.push_head(2);
    list = list.push_head(3);

    println!("{}", list.display());
}