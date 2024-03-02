A very (not) safe library to check if a number is odd. 
Abuses the same bugs and tricks in cve-rs to transmute
the given `i8` to a `bool` without using `unsafe`. 

This relies on a compiler bug, almost certainly doesn't
work on all platforms, and is possibly the least
reasonable `is_odd` function you could imagine. 

See [https://www.youtube.com/watch?v=vfMpIsJwpjU&list=PLzl2iy0KCGD6N93omgPEjgakVYA5t-1oV](https://www.youtube.com/watch?v=vfMpIsJwpjU&list=PLzl2iy0KCGD6N93omgPEjgakVYA5t-1oV)

Example:
```rust
use odd_is_odd::is_odd;
fn main() {
    if (is_odd(69)) {
        println!("on our way to hundreds of thousands of downloads a week!");
    } else {
        println!("who could have guessed, the sketchy library is sketchy");
        assert!(false);
    }
}
```
