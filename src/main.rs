#![feature(test)]

use rust_algos_illuminated::chapter_one::{RecIntMul, Karatsuba};

mod tests;

fn main() {
    let mul = RecIntMul(12, 12);
    let kmul = Karatsuba(12, 12);
    println!("{} = {}", mul, kmul);
}
