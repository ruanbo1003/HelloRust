
pub fn tests() {
    let x = 10;
    let y = 20;

    let single = Borrowed(&x);
    let double = NamedBorrowed{x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("{:?}, {:?}, {:?}, {:?}", single, double, reference, number);
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
