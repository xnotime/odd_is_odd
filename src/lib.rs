//! A very (not) safe library to check if a number is odd. 
//! Abuses the same bugs and tricks in cve-rs to transmute
//! the given `i8`` to a `bool` without using `unsafe`. 
//! 
//! This relies on a compiler bug, almost certainly doesn't
//! work on all platforms, and is possibly the least
//! reasonable `is_odd` function you could imagine. 
//!
//! See [https://www.youtube.com/watch?v=vfMpIsJwpjU&list=PLzl2iy0KCGD6N93omgPEjgakVYA5t-1oV](https://www.youtube.com/watch?v=vfMpIsJwpjU&list=PLzl2iy0KCGD6N93omgPEjgakVYA5t-1oV)
//!
//! Example
//! ```
//! use odd_is_odd::is_odd;
//! fn main() {
//!     if (is_odd(69)) {
//!         println!("on our way to hundreds of thousands of downloads a week!");
//!     } else {
//!         println!("who could have guessed, the sketchy library is sketchy");
//!         assert!(false);
//!     }
//! }
//! ```

#![forbid(unsafe_code)]

use std::hint::black_box;

/// Checks if an `i8` is odd. Read the crate docs. 
/// Using this is a terrible, terrible idea. 
pub fn is_odd(number: i8) -> bool {
    definitely_not_transmute(number)
}

enum Sneaky<S, T> {
    From(Option<Box<S>>),
    To(Option<Box<T>>),
}

#[inline(never)]
fn definitely_not_transmute<S, T>(input: S) -> T {
    let mut sneaky: Sneaky<S, T> = Sneaky::To(None);
    let outer = &mut sneaky;

    let inner = match outer {
        Sneaky::To(something) => something,
        Sneaky::From(_) => unreachable!(),
    };
    let inner = expand_mut(inner);

    *outer = Sneaky::From(Some(Box::new(input)));
    black_box(outer);

    *inner.take().unwrap()
}

//   WATCH YOUR STEP!  
// SOUNDNESS HOLE BELOW

fn weird<'a, 'b, T>(_witness: &'b &'a (), borrow: &'a mut T) -> &'b mut T {
    borrow
}

const FOREVER: &'static &'static () = &&();

fn expand_mut<'a, 'b, T>(borrow: &'a mut T) -> &'b mut T {
    let converted: fn(&'b &'static (), &'a mut T) -> &'b mut T = weird;
    converted(FOREVER, borrow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_numbers() {
        assert_eq!(is_odd(1), true);
        assert_eq!(is_odd(2), false);
        assert_eq!(is_odd(3), true);
        assert_eq!(is_odd(42), false);
        assert_eq!(is_odd(69), true);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(is_odd(-1), true);
        assert_eq!(is_odd(-2), false);
        assert_eq!(is_odd(-3), true);
        assert_eq!(is_odd(-42), false);
        assert_eq!(is_odd(-69), true);
    }

    #[test]
    fn zero() {
        assert_eq!(is_odd(0), false);
    }

    #[test]
    fn extremes() {
        assert_eq!(is_odd(127), true);
        assert_eq!(is_odd(-128), false);
    }
}
