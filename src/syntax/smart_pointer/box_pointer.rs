
/*
box( Box<T>) put the variable in heap, and put the pointer that pointed
to the variable on stack.

Use cases:
  1. a data type that don't known the size at compile time, and want to use
    it in a context where exact size if needed.
  2. big size of data and want to transfer ownership without copy.
  3. use a type when just care about if it implemented the specified trait,
    not care about its own type.
 */

pub fn tests() {
    int_box();
}


/*
just an example, not a good use.
 */
fn int_box() {
    let b = Box::new(10);
    println!("{}, {:}", b, &b);  // 5, 5

}  // b leave its scope, it will release the box itself(in stack) and the data(in heap) it pointed at.


//  error declares
// error: recursive type `ErrList` has infinite size.
// enum ErrList {
//     ECons(i32, ErrList),  // recursive without indirection
//     ENil,
// }
// end of error declares

enum CorrectList {
    Cons(i32, Box<CorrectList>),
    Nil
}
fn list_test() {
    let list = CorrectList::Cons(1,
        Box::new(CorrectList::Cons(2,
        Box::new(CorrectList::Cons(3,
        Box::new(CorrectList::Nil))))));
}