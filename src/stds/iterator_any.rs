
pub fn tests() {
    // vector iterator any
    {
        let v1 = vec![1, 2, 3];
        println!("{}", v1.iter().any(|&x|x==1));  // true

        let v2 = vec![7, 8, 9];
        println!("{}", v2.iter().any(|&x|x==1));  // false
    }

    // array iterator any
    {
        let a1 = [1, 2, 3];
        let a2: [i32; 3] = [7, 8, 9];
        println!("{}", a1.iter().any(|&x|x == 2));  // true
        println!("{}", a2.iter().any(|&x|x == 2));  // false
    }
}

// annotation of iterator::any
pub trait MyIterator {
    type Item;

    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool {
        true
    }
}