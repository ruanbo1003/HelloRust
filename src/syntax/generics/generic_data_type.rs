#![allow(dead_code)]

pub fn tests() {

    {
        let number_list = vec![1, 2, 3, 10, 5];
        let largest_i32 = largest_i32(&number_list);

        let char_list = vec!['a', 'b', 'x', 'z'];
        let largest_char = largest_char(&char_list);

        println!("largest i32:{}, largest char:{}", largest_i32, largest_char);
    }

    generic_struct();
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // for item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    return largest;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn generic_struct() {
    let both_int = Point {x:5, y:10};
    let both_float = Point{x: 1.0, y:1.0};
    println!("{:?}, {:?}", both_int, both_float);

    let two_type_point = Point2{x:1, y:2.2};
    println!("{:?}", two_type_point);
}