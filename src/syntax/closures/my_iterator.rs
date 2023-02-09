#![allow(dead_code, unused_variables)]

/*
implement a iterator
 */
pub fn tests() {
    // test my Counter
    {
        let mut counter = Counter::new();
        println!("{:?}", counter.next());  // Some(1)
        println!("{:?}", counter.next());  // Some(2)
        println!("{:?}", counter.next());  // Some(3)
        println!("{:?}", counter.next());  // Some(4)
        println!("{:?}", counter.next());  // Some(5)
        println!("{:?}", counter.next());  // None
        println!("{:?}", counter.next());  // None
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    // new() is private.
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        // from 1 to 5
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}