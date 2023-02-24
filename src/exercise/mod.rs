#![allow(dead_code, unused_variables)]

pub fn tests() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point = Point {
        x: 0, y: 0,
    };
    let b1 = &point;
    let b2 = &point;
    println!("{:?}, {:?}", b1, b2);

    point.x = 1;
    println!("{:?}", point);

    let b3 = &mut point;
    println!("{:?}", b3);  // Point { x: 1, y: 0 }

    let b4 = &point;
    println!("{:?}", b4);
}

