#![allow(non_snake_case)]
// (names of functions are the same as names
//  of algorithms in the book)

extern crate test;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use rust_algos_illuminated::chapter_one::{
        RecIntMul, Karatsuba, IntMul};

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

    #[bench]
    fn bench_chap1_mult_std(b: &mut Bencher) {
        b.iter(|| IntMul(12, 13));
    }

    use rust_algos_illuminated::chapter_one::{selectionSort, isSorted, MergeSelectionSort};

    #[test]
    fn test_isSorted() {
        assert_eq!(isSorted(&[2i32, 4i32, 1i32 ,3i32].to_vec()), false);
        assert_eq!(isSorted(&[1i32, 2i32, 3i32, 4i32].to_vec()), true);
        assert_eq!(isSorted(&[35i32, 12i32, 36i32, 37i32].to_vec()), false);
    }

    #[test]
    fn test_quadSort() {
        assert_eq!(selectionSort([2i32, 4i32, 1i32 ,3i32].to_vec()),
            [1i32, 2i32, 3i32, 4i32].to_vec());
    }

    #[test]
    fn test_MergeSelectionSort() {
        assert_eq!(MergeSelectionSort([2i32, 4i32, 1i32 ,3i32, 5i32, 6i32, 7i32, 8i32].to_vec()),
            [1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32].to_vec());
    }

}
