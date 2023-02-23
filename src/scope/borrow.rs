
/*
In many cases, we hope we can access variable, but not take the ownership.
To implement this, Rust use `borrow`, variable can use reference(&T) to pass,
instead of value(T)
 */

pub fn tests() {
    // compare_of_move_and_borrow();

    mutable_reference();
}

fn compare_of_move_and_borrow() {
    fn task_ownership_box_i32(b: Box<i32>) {
    }
    fn ref_borrow_box_i32(b: &Box<i32>) {
    }

    {
        let b1 = Box::new(1);
        task_ownership_box_i32(b1);
        // error: borrowed of moved value: `b1`
        // println!("{}", b1);  // error

        let b2 = Box::new(2);
        ref_borrow_box_i32(&b2);
        println!("{}", b2);  // ok
    }
}

fn mutable_reference() {
    #[derive(Debug)]
    struct Book {
        count: i32,
    }

    fn immutable_borrow_book(book: &Book) {
        println!("borrow_book:{:?}", book);
        // error: Cannot assign to field of immutable binding
        // book.count = -1;  // error
    }
    fn mutable_borrow_book(book: &mut Book) {
        book.count = -1;
    }

    // immutable variable
    {
        let book = Book{count: 1};

        // immutable borrow of a immutable variable
        immutable_borrow_book(&book);

        // mutable borrow of a immutable variable
        // mutable_borrow_book(&book); // error

        // error: Cannot borrow immutable local variable `book` as mutable.
        // mutable_borrow_book(&mut book); // error
    }

    // mutable variable
    {
        let mut book = Book { count: 1};

        // immutable borrow of mutable variable: OK
        immutable_borrow_book(&book);

        // mutable borrow of mutable variable: OK
        mutable_borrow_book(&mut book);
    }

}