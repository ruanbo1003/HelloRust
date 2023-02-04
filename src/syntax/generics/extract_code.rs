#![allow(dead_code)]

use crate::syntax::enums::enum_tests;

pub fn extract() {
    largest_number();

}

fn largest_number() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("the largest number is {}", largest);
    }

    {
        let number_list = vec![1, 2, 3, 100, 200, 0, 10];
        let mut largest = &number_list[0];
        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("the largest number is {}", largest);
    }

    {
        // use a extract function: largest()
        let number_list_1 = vec![34, 50, 25, 100, 65];
        let largest_1 = largest(&number_list_1);

        let number_list_2 = vec![1, 2, 3, 100, 200, 0, 10];
        let largest_2 = largest(&number_list_2);

        println!("largest number is {}, {}", largest_1, largest_2);
    }
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}