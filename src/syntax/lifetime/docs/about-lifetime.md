
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


 



