
### variable can be used after assign to another
* example
```shell
{
    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);
}
```
1. Both x and y are valid after the statement `let y = x;`.
Rust has a special trait call `Copy trait`, which call be used types like 
integer, those type call be stored in `stack`.  
2. If a type implement the `Copy trait`, then the variable of this type is 
still valid after assigned to other variable, like the `x` above.
3. If a type implement the `Drop trait`, it cannot use the `Copy` trait.
4. types that implemented `Copy trait`? any combines of `simple scalar type` 
can implement `Copy trait`, any type that don't need to allocate memory or other 
resource can implement `Copy trait` too. Such as:
   * integers, i8, u8, i16, u16 ... i128, u128
   * boolean, bool
   * float, f32, f64,
   * char
   * tuple when elements are implemented `Copy trait`, (i32, i32), (i32, char).




