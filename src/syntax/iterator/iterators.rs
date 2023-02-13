#![allow(dead_code, unused_variables)]

pub fn tests() {
    basic_usage_of_iterator();

    // use_iterator();
}

fn basic_usage_of_iterator() {
    // the lazy of iterator
    {
        let v1 = vec![1, 2, 3];
        let it = v1.iter();  // there is nothing to do with this code.
    }

    // use `for` with iterator
    {
        let v = vec![1, 2, 3];
        let it = v.iter();
        for each in it {
            print!("{} ", each);
        }  // 1 2 3
        println!();
    }

    // the `next` function of iterator
    {
        let v = vec![1, 2, 3];
        let mut it = v.iter();
        println!("v[0]:{:?}", it.next());  // Some(1)
        println!("v[1]:{:?}", it.next());  // Some(2)
        println!("v[2]:{:?}", it.next());  // Some(3)
        println!("next:{:?}", it.next());  // None

        // the reason why the output is Some(1), not 1:
        /*
        pub trait Iterator {
            type Item;

            // the return type of `next()` is Option<Self::Item>
            fn next(&mut self) -> Option<Self::Item>;
        }
         */
    }
}




fn use_iterator() {
    // sum
    {
        let v = vec![1, 2, 3];
        let it = v.iter();
        let sum: i32 = it.sum();
        println!("sum:{}", sum);  // 6
    }

    // map
    {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_>= v1.iter().map(|x|x+1).collect();
        println!("v2:{:?}", v2);  // [2, 3, 4]
        println!("v1:{:?}", v1);  // [1, 2, 3]
    }

    // filter
    println!("\niterator filter");
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter()
                .filter(|s| s.size == shoe_size)
                .collect()
        }

        let shoes = vec![
            Shoe { size: 10, style: String::from("red") },
            Shoe { size: 13, style: String::from("green") },
            Shoe { size: 10, style: String::from("gray") },
            Shoe { size: 12, style: String::from("yellow") },
        ];

        let size_10 = shoes_in_size(shoes, 10);
        println!("size in 10:{:?}", size_10);

        // error: use of moved value
        // let size_12 = shoes_in_size(shoes, 12);

    }
}
