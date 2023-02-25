

/*
normally, struct methods don't need lifetime annotation,
because the lifetime of `self` will be assigned to all of the
output lifetime, by default.
 */

pub fn tests() {
    let mut own = Owner(1);
    own.add_one();
    own.print(); // 2
}

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("{}", self.0);
    }
}