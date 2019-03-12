#![allow(non_snake_case)]
// (names of functions are the same as names
//  of algorithms in the book)

extern crate test;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use rust_algos_illuminated::chapter_one::{RecIntMul, Karatsuba};

    #[test]
    fn test_RecIntMul() {
        assert_eq!(RecIntMul(2, 3), (2 * 3));
    }

    #[test]
    fn test_Karatsuba() {
        assert_eq!(Karatsuba(2, 3), (2 * 3));
    }

    #[bench]
    fn bench_chap1_mult_recursive(b: &mut Bencher) {
        b.iter(|| RecIntMul(12, 13));
    }

    #[bench]
    fn bench_chap1_mult_karat(b: &mut Bencher) {
        b.iter(|| Karatsuba(12, 13));
    }

}