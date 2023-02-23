
/*
Variable need to release the resource it owned, so resource can have only one owner.
This also avoid duplicate release.
Not all variable own resource, like reference.

When assign one variable to another(let x = y;),
or pass value to function (foo(z)), the resource ownership will transfer.
This call resource `move` in Rust.

After `move`, the original owner cannot be used anymore, this can avoid
dangling pointer.
 */

pub fn tests() {
    // variable_copy_and_move();

    // mutable_when_ownership_move();

    partial_move();
}

fn variable_copy_and_move() {
    // basic type assigment will make a copy.
    {
        let x = 1i32;
        let y = x;
        println!("{}, {}", x, y);  // 1, 1
    }

    {
        let a = Box::new(10);
        println!("a:{}", a);  // 10

        let b = a;

        // a cannot be used anymore, because the ownership of if resource is moved to b.
        // println!("a:{}", a); // error: borrow of moved value `a`

        println!("b:{}", b);  // 10
    }

    {
        // variable of type Box<i32> pass to function,
        // the function will take ownership of the variable.
        fn release_box(c: Box<i32>) {}

        let a = Box::new(100);
        release_box(a);

        // `a` cannot be used anymore, because the ownership is moved into the function.
        // println!("a:{}", a);  // error: borrow of moved value: `a`
    }
}

/*
the mutable of the variable can be changed during the ownership move
 */
fn mutable_when_ownership_move() {
    let b = Box::new(1);

    // error
    // *b = 10;  // error: cannot assign to `*b`, as `b` is not declared as mutable.

    let mut b2 = b;
    *b2 = 10;

    println!("b2:{}", &b2);  // 10
}

/*
When destruct a variable, we can destruct by-move, and by-reference at the same time.
This means partial of the field is moved, the others not.
 */
fn partial_move() {
    #[derive(Debug)]
    struct Person {
        first: String,
        last: String,
    }
    let person = Person {
        first: String::from("Jack"),
        last: String::from("Tom"),
    };

    // first: by move
    // ref last: by reference
    let Person {first, ref last } = person;
    println!("{}, {}", first, last);

    // println!("{:?}", person);  // error: borrow of partially moved value `person`
    // println!("first:{}", person.first);  // error: borrow of moved value `person.first`
    println!("last:{}", person.last);
}