

/*
Multiple variables can interact with the same data in different ways.
 */

pub fn tests() {
    // interact_with_move();

    // interact_with_clone();

    stack_only_data_interact_with_copy();
}


fn interact_with_move() {
    // String move
    {
        let s1 = String::from("hello");
        let s2 = s1;  // value moved here.
        println!("s2:{}", s2);  // hello

        // s1 is moved to s2, and s1 is invalid now.
        //println!("s1:{}", s1);  // error: borrow of moved value `s1`
    }
}

fn interact_with_clone() {
    // String clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();  // clone s1, and assign the cloned value to s2.

        println!("s2:{}", s2);  // hello
        println!("s1:{}", s1);  // hello
    }
}

fn stack_only_data_interact_with_copy() {
    // see readme.md
    // integer variable still valid after assigned to another, without clone.
    {
        let i1 = 10;
        let i2 = i1;
        println!("i2:{}", i2);

        // unlike the `x1, x2` in interact_with_clone. we don't clone i1, and assign i1 to i2.
        // i1 still valid after the assignment.
        println!("i1:{}", i1);
    }

    {
        let item = UserItemNoCopy{count:1, price:1.1};
        println!("{:?}", item);  // UserItemNoCopy { count: 1, price: 1.1 }

        let item2 = item;
        println!("{:?}", item2);  // // UserItemNoCopy { count: 1, price: 1.1 }

        // error
        // println("{:?}", item)  // Use of moved value
    }

    {
        let item1 = UserItemWithCopy{count:2, price:2.2};
        println!("{:?}", item1);  // UserItemWithCopy { count: 2, price: 2.2 }

        let item2 = item1;
        println!("{:?}", item2);  // UserItemWithCopy { count: 2, price: 2.2 }

        // the struct `UserItemWithCopy` has a marker `Clone` and `Copy`.
        // so after declare `item2` and assign `item1` to `item2`,  `item1` is still valid.
        println!("{:?}", item1)  // UserItemWithCopy { count: 2, price: 2.2 }
    }
}


#[derive(Debug)]
struct UserItemNoCopy {
    count: i32,
    price: f32,
}

#[derive(Copy, Clone, Debug)]
struct UserItemWithCopy {
    count: i32,
    price: f32,

    // error if the struct has a `String` member variable
    // name: String,  // this field does not implement 'Copy'
}
