#![allow(dead_code)]

pub fn tests() {
    string_slice();

    // get first word of text
    {
        let mut s1 = String::from("hello");
        let first_1 = first_world(&s1);
        println!("first world:{first_1}");  // hello

        s1.clear();  // ok
        println!("s1 after clear():{}", s1);  // empty

        let s2 = String::from("hello rust");
        let first_2 = first_world(&s2);

        // s2.clear(); // error: cannot borrow `s2` as mutable because it is also borrowed as immutable

        println!("first world:{first_2}");  // hello
    }

    // slice of array
    {
        array_slice_1();
    }

    // update array element by using slice
    {
        change_array_element_by_using_alice()
    }
}

fn string_slice() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}-{world}, {}", s.len());  // hello-world 11
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn array_slice_1() {
    let a1 = [1, 2, 3, 4, 5];
    let s1 = &a1[1..3];
    println!("is equal: {}", s1 == &[2, 3])  // true
}

fn change_array_element_by_using_alice_error_1() {
    // let mut a = [1, 2, 3];
    // let s = &mut a[1..2];  // mutable borrow
    // println!("{:?}", a);  // immutable borrow
    // s[0] = 20;  // mutable borrow later used here
}

fn change_array_element_by_using_alice() {
    let mut a = [1, 2, 3];
    let s = &mut a[1..2];  // mutable borrow
    s[0] = 20;

    // after change s, the value of a is also changed.
    println!("{:?}", a)  // [1, 20, 3]
}