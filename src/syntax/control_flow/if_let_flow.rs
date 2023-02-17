
pub fn tests() {
    // if-let with `Some`
    {
        let number = Some(7);
        let letter: Option<i32> = None;
        let emo: Option<i32> = None;

        if let Some(i) = number {
            println!("i:{}", i);  // 7
        }

        if let Some(i) = letter {
            println!("matched letter");
        } else {
            // not-matched
            println!("not-matched a letter");
        }
    }

    // if-let with `enum`
    {
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(1);

        if let Foo::Bar = a {
            println!("a-Bar");
        }

        if let Foo::Bar = b {
            println!("b-Bar");
        }

        if let Foo::Qux(val) = c {
            println!("c:{}", val);  // 1
        }

    }
}