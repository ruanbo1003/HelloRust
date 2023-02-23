
/*
RAII: Resource Acquisition Is Initialization.
Rust execute RAII enforce, when variable leaves out of scope, the `destructor`
will be code.
This can avoid `resource leak`
 */

pub fn tests() {
    raii_release();

    raii_drop_trait();
}

fn raii_release() {
    fn create_box() {
        let _box = Box::new(10);
        // when `_box` out of scope, the `destructor` of Box will be called, to release resource.
    }

    let _box1 = Box::new("1");

    {
        let _box2 = Box::new("2");
    }
}

fn raii_drop_trait() {
    struct Item {
        data: i32,
    }
    impl Drop for Item {
        fn drop(&mut self) {
            println!("Item::drop({})", self.data);
        }
    }

    {
        let item = Item { data: 1 };
    }  // Item::drop(1)

    {
        let item = Item { data: 2 };
    }  // Item::drop(2)
}