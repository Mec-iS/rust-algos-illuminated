#![feature(test)]
//! ************************************************
//! A collection of algorithms as presented in
//! the book "Algorithms Illuminated Part 1" by Tim
//! Roughgarden.
//! -----------------------------------------------
//! ************************************************
//! (names of functions are the same as names
//!  of algorithms in the book)
mod chapter_one;

use chapter_one::{RecIntMul, Karatsuba};

mod tests;

fn main() {
    let mul = RecIntMul(12, 12);
    let kmul = Karatsuba(12, 12);
    println!("{} = {}", mul, kmul);
}
