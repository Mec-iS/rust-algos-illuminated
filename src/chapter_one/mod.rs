#![allow(non_snake_case)]
//! *********************************
//! Module for Chapter One Algorithms
//! *********************************

/// Standard library multiplication
#[no_mangle]
pub extern fn IntMul(x: u64, y: u64) -> u64 {
    // PARAGRAPH 1.2.2
    // implementation from STD
    // (integers have same lenght n)
    x * y
}

/// PARAGRAPH 1.3.2
/// a recursive implementation by
/// slicing into shorter integers (n/2-digits integers)
/// (integers have same lenght n)
#[no_mangle]
pub extern fn RecIntMul(x: u64, y: u64) -> u64 {
    // x * y = 10^n * (a*c) + 10^n/2 * (a*d+b*c) + (b*d)

    let n = x.to_string().len();

    if n == 1 {
        return x * y;
    }

    //  split integers into shorter: a b c d
    let a: u64 = x.to_string()[0..(n/2)].parse::<u64>().unwrap();
    let b: u64 = x.to_string()[(n/2)..].parse::<u64>().unwrap();
    let c: u64 = y.to_string()[0..(n/2)].parse::<u64>().unwrap();
    let d: u64 = y.to_string()[(n/2)..].parse::<u64>().unwrap();

    let n = n as u32;

    u64::pow(10, n) * RecIntMul(a, c) +
    u64::pow(10, n/2) * (RecIntMul(a, d) + RecIntMul(b, c)) +
    RecIntMul(b, d)

}

/// PARAGRAPH 1.3.3
/// Optimized version of RectIntMul as
/// discovered by Karatsuba.
/// Make only three recursive calls.
#[no_mangle]
pub extern fn Karatsuba(x: u64, y: u64) -> u64 {

    let n = x.to_string().len();

    if n == 1 {
        return x * y;
    }

    //  split integers into shorter: a b c d
    let a: u64 = x.to_string()[0..(n/2)].parse::<u64>().unwrap();
    let b: u64 = x.to_string()[(n/2)..].parse::<u64>().unwrap();
    let c: u64 = y.to_string()[0..(n/2)].parse::<u64>().unwrap();
    let d: u64 = y.to_string()[(n/2)..].parse::<u64>().unwrap();

    let p = a + b;
    let q = c + d;

    let n = n as u32;

    let ac = Karatsuba(a, c); let bd = Karatsuba(b, d);

    u64::pow(10, n) * ac +
    u64::pow(10, n/2) * (Karatsuba(p, q) -
    ac - bd) +
    bd

}

/// Paragraph 1.4 Sorting
/// Check if an array is sorted
#[no_mangle]
pub extern fn isSorted(arr: &Vec<i32>) -> bool {
    if arr.len() == 1 || arr.len() == 0 {
        // base case: array is already ordered
        return true
    }

    let n = arr.len();
    if n == 1 {
        return true
    }

    for (i, el) in arr.iter().enumerate() {
        if i+1 > n-1 {
            // loop reached the end
            return true
        }
        if el > &arr[i+1] {
            return false
        };
    }
    true
}


/// Paragraph 1.4 Sorting
/// Sort an array in quadratic time
#[no_mangle]
pub extern fn selectionSort(arr: Vec<i32>) -> Vec<i32> {
    if isSorted(&arr) {
        // base case: array is already ordered
        return arr
    }

    //let mut vec = arr;
    // vec.sort();

    let n = arr.len();
    let mut swap_arr: Vec<i32> = arr.clone();

    // advance the position through the entire array
    for j in 0..n-1 {
        // assume the min is the first element
        let mut i_min: usize = j as usize;

        // test against elements after j to find the smallest
        for i in j+1..n {
            if swap_arr[i] < swap_arr[i_min] {
                // if this element is less, then it is the new minimum
                i_min = i as usize
            }
        }

        if i_min != j {
            swap_arr.swap(j, i_min as usize);
        }

    }

    swap_arr
}

/// PARAGRAPH 1.4.4
/// Basic Merge-Sort for two arrays
#[no_mangle]
pub extern fn MergeSelectionSort(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();

    if n == 0 || n == 1 || isSorted(&arr) {
        return arr
    }

    if n < 3 {
        return selectionSort(arr)
    }

    let n = arr.len() as f32;
    let i: usize = (n/2 as f32).floor() as usize;

    // divide the input
    let (a, b) = arr.split_at(i);

    let divide_first = selectionSort(a.to_vec());
    let divide_second = selectionSort(b.to_vec());

    return Merge(divide_first, divide_second)

}

/// Paragraph 1.4.5
/// Merge subroutine
#[no_mangle]
pub extern fn Merge(c: Vec<i32>, d: Vec<i32>) -> Vec<i32> {
    let l: usize = c.len() + d.len();

    // initialize result array
    let mut b: Vec<i32> = vec!(0; l);

    // initialize readers indexes
    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in 0..l {
        // get value or None from the two array
        let (c_value, d_value) = (c.get(i), d.get(j));

        match (c_value, d_value) {
            // reading arrays are done
            (None, None) => break,
            // d is larger, keep going
            (None, Some(_)) => {
                b[k] = d[j];
                j += 1;
            },
            // c is larger, keep going
            (Some(_), None) => {
                b[k] = c[i];
                i += 1;
            },
            // generic case, keep going
            (Some(_), Some(_)) =>  match c_value.unwrap() < d_value.unwrap() {
                true => {
                    b[k] = c[i];
                    b[k+1] = d[j];
                    i += 1;
                },
                false => {
                    b[k] = d[j];
                    b[k+1] = c[i];
                    j += 1;
                }
            }
        }
    }

    return b

}


// fn quicksort<T: Ord>(slice: &mut [T]) {
//     if slice.len() <= 1 {
//         return; // Nothing to sort.
//     }
//     // Partition the slice into two parts, front and back.
//     let pivot_index = partition(slice);
//     // Recursively sort the front half of `slice`.
//     quicksort(&mut slice[.. pivot_index]);
//
//     quicksort(&mut slice[pivot_index + 1 ..]);
// }
