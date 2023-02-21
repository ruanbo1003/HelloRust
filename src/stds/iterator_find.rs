
pub fn tests() {
    // vector iterator find
    {
        let v1 = vec![1, 2, 3, 4, 5];
        let mut iter1 = v1.iter();  // iter, type of `&i32`

        // the reference of iterator item if `&&i32`, destructure to `i32`
        // the `find` will pass the reference of iterator item to closure, iterator reference type
        // is `&i32`, the reference of iterator item is `&&i32`.
        println!("{:?}", iter1.find(|&&x|x == 3));  // Some(3)
        println!("{:?}", iter1.find(|&&x| x == 0));  // None

        let v2 = vec![9, 8, 7, 6, 5];
        let mut into_iter = v2.into_iter();  // into_iter, type of `i32`
        println!("{:?}", into_iter.find(|&x| x == 9));  // Some(9)
        println!("{:?}", into_iter.find(|&x| x == 0));  // None
    }

    // array iterator find
    {
        let a1 = [1, 2, 3];
        println!("{:?}", a1.iter().find(|&&x| x == 1));  // Some(1)
        println!("{:?}", a1.into_iter().find(|&x|x==0));  // None

        let a2 :[i32;3] = [7, 8, 9];
        println!("{:?}", a2.iter().find(|&&x|x==8));  // Some(8)
        println!("{:?}", a2.into_iter().find(|&x|x==0));  // None
    }
}

// annotation of iterator::find
pub trait MyIterator {
    type Item;

    // parameter type is `&mut self`, can borrow and update.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut`: captured variables can be update, no release.
        // `&Self::Item`: captured variables type(the reference type of iterator items)
        P: FnMut(&Self::Item) -> bool {
        None
    }
}