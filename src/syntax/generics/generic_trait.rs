
pub fn tests() {
    let e = Empty;
    let n = Null;

    e.double_drop(n);

    // e;  // error: use of moved value: `e`
    // n;  // error: use of moved value `n`
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T){
        println!("double_drop");
    }
}

