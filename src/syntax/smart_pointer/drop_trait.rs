
pub fn tests() {
    when_drop_trait_execute();

}

fn when_drop_trait_execute() {
    // one instance
    {
        let p = TestSmartPointer{data:String::from("ABC")};
    }  // ABC

    // two instances
    {
        let p1 = TestSmartPointer{ data: String::from("abc") };
        let p2 = TestSmartPointer{ data: String::from("xyz") };

    }  // xyz  abc

    // force call drop trait before variable out-of-scope
    {
        let p1 = TestSmartPointer{ data: String::from("hello") };
        drop(p1);  // hello
    } // the drop trait will not be called here.
    println!("after force call drop scope");
}

struct TestSmartPointer {
    data: String,
}

impl Drop for TestSmartPointer {
    fn drop(&mut self) {
        println!("Dropping TestSmartPointer:{}", self.data);
    }
}