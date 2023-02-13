
## about lifetime
* Each reference in Rust has his lifetime, which is the 
scope where the reference keep valid.
* Most of the time, the lifetime of variable can be
 inference, just like the type of the variable.
* And like we must specify the variable type when 
 there might be multiple types. In some situation, 
 we must specify the lifetime of the reference.

## purpose of lifetime
* the main goal of lifetime is to avoid the use of dangling-reference.

## borrow-checker
 We have a variable named `x`, and another variable named `r` which is 
 the reference of `x`. 
```rust
let x = 10;
let r = &x;
println!("r:{}", r);
```
 The borrow-checker make sure the reference `r` if valid by compare the
 valid-scope of the variable and the reference.
* the variable's valid-scope must bigger than the variable reference's scopy.


## lifetime elision rules
* input lifetimes & output lifetimes
  - function or member function's argument lifetime is called `input lifetimes`.
  - lifetime of return value is called `output lifetimes`.
* three roles of lifetime-elision
  1. Every reference parameter has its own lifetime parameter. For example, function foo has one reference parameter,
  than it has one lifetime parameter, function bar has two reference parameters, than it has two lifetime parameters.
  And so on.
      ```rust
      fn foo(s: &str, i: i32) {}
      fn bar(s1: &str, s2: &str, i: i32){}
       ```
  2. If a function has one input-lifetime parameter, then all its output-lifetime is the same as input-lifetime.
      ```rust
       fn foo(s: &str) -> &str {}
       fn foo_2<'a>(s: &'a str) -> &'a str {}
     ```
  3. if a struct method has multiple input-lifetime, and one of them is `&self` or `&mut self`,
     than all output-lifetime is assigned by the lifetime of `self`.




