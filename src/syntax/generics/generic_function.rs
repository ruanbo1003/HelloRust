#![allow(dead_code)]

pub fn tests() {

    // different function for different parameter type
    {
        let number_list = vec![1, 2, 3, 10, 5];
        let largest_i32 = largest_i32(&number_list);

        let char_list = vec!['a', 'b', 'x', 'z'];
        let largest_char = largest_char(&char_list);

        println!("largest i32:{}, largest char:{}", largest_i32, largest_char);
    }
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
    let largest = &list[0];

    // for &item in list.iter() {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    return largest;
}
