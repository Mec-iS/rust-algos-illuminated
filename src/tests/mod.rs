#![allow(non_snake_case)]
// (names of functions are the same as names
//  of algorithms in the book)

extern crate test;  // load test crate

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::chapter_one::{
        RecIntMul, Karatsuba, IntMul,
        selectionSort, isSorted, MergeSelectionSort};

    use crate::other::Queue;

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

    #[test]
    fn test_queue_raw() {
        let mut q = Queue { q_out: Vec::new(), q_in: Vec::new() };
        q.push('0');
        q.push('1');
        assert_eq!(q.pop_raw(), Some('0'));
        q.push('∞');
        assert_eq!(q.pop_raw(), Some('1'));
        assert_eq!(q.pop_raw(), Some('∞'));
        assert_eq!(q.pop_raw(), None);
    }

    #[test]
    fn test_queue_swap() {
        let mut q = Queue { q_out: Vec::new(), q_in: Vec::new() };
        q.push('0');
        q.push('1');
        assert_eq!(q.pop_swap(), Some('0'));
        q.push('∞');
        assert_eq!(q.pop_swap(), Some('1'));
        assert_eq!(q.pop_swap(), Some('∞'));
        assert_eq!(q.pop_swap(), None);
    }

    #[bench]
    fn bench_queue_pop_raw(b: &mut Bencher) {
        b.iter(|| {
            let mut q = Queue::new();
            q.push('0');
            q.push('1');
            q.push('字');
            q.pop_raw();
            q.push('∞');
            q.pop_raw();
            q.pop_swap();
            q.pop_raw();
            q.pop_raw();
        });
    }

    #[bench]
    fn bench_queue_pop_swap(b: &mut Bencher) {
        b.iter(|| {
            let mut q = Queue::new();
            q.push('0');
            q.push('1');
            q.push('字');
            q.pop_swap();
            q.push('∞');
            q.pop_swap();
            q.pop_swap();
            q.pop_swap();
            q.pop_swap();
        });
    }

}
