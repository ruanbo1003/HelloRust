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

fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn specific_generic_function_type() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    // parameter type: S, regular function, not generic function
    fn regular_fn(_s: S) {
    }

    // parameter: specify the SGen<A> type explicit.
    fn gen_spec_a(_s: SGen<A>) {
        println!("gen_spec_a");
    }
    fn gen_spec_i32(_s: SGen<i32>) {
        println!("gen_spec_i32");
    }

    fn gen_spec_f64(_s: SGen<f64>) {
        println!("gen_spec_f64");
    }

    // parameter type: generic SGen<T>
    fn gen<T>(_s: SGen<T>) {
        println!("gen")
    }

    {
        regular_fn(S(A));

        gen_spec_a(SGen(A));

        gen_spec_i32(SGen(10));

        gen_spec_f64(SGen(1.0));

        gen::<char>(SGen('a'));

        gen(SGen("hello"));
    }
}