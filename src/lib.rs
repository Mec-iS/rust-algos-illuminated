#![allow(non_snake_case)]
// (names of functions are the same as names
//  of algorithms in the book)

///
/// A collection of algorithms as presented in 
/// the book "Algorithms Illuminated Part 1" by Tim 
/// Roughgarden.
/// -----------------------------------------------
///

pub mod chapter_one {

    /// Standard library multiplication
    pub fn IntMul(x: u64, y: u64) -> u64 {
        // PARAGRAPH 1.2.2
        // implementation from STD
        // (integers have same lenght n)
        x * y
    }

    // PARAGRAPH 1.3.2
    // a recursive implementation by 
    // slicing into shorter integers (n/2-digits integers)
    // (integers have same lenght n)
    pub fn RecIntMul(x: u64, y: u64) -> u64 {
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
    pub fn Karatsuba(x: u64, y: u64) -> u64 {

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

}