//! Kata "Disease Spread":
//! http://www.codewars.com/kata/566543703c72200f0b0000c9


fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32{
    // your code
}


#[cfg(test)]
fn dotest(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64, exp: i32) -> () {
    assert_eq!(exp, epidemic(tm, n, s0, i0, b, a))
}

#[test]
fn basic_tests() {
    dotest(18, 432, 1004, 1, 0.00209, 0.51, 420);
    dotest(12, 288, 1007, 2, 0.00206, 0.45, 461);
    dotest(13, 312, 999, 1, 0.00221, 0.55, 409);
}
