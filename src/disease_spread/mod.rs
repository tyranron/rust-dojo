//! Kata "Disease Spread":
//! http://www.codewars.com/kata/566543703c72200f0b0000c9


/// An imperative solution for "Disease Spread" problem.
/// Counts and returns the maximum number of infecteds.
fn epidemic_imper(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let (mut s, mut i, mut max) = (s0 as f64, i0 as f64, i0 as f64);
    let dt = tm as f64 / n as f64;
    for _ in 0..n {
        let s_prev = s;
        s = s - dt * b * s * i;
        i = i + dt * (b * s_prev * i - a * i);
        if i > max {
            max = i;
        }
    }
    max as i32
}


/// A functional solution for "Disease Spread" problem.
/// Counts and returns the maximum number of infecteds.
fn epidemic_funct(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let dt = tm as f64 / n as f64;
    (0..n)
        .fold((s0 as f64, i0 as f64, i0 as f64), |(s, i, max), _| {
            (s - dt * b * s * i,
             i + dt * (b * s * i - a * i),
             if i > max { i } else { max })
        })
        .2 as i32
}


#[test]
fn basic_tests_imper() {
    assert_eq!(420, epidemic_imper(18, 432, 1004, 1, 0.00209, 0.51));
    assert_eq!(461, epidemic_imper(12, 288, 1007, 2, 0.00206, 0.45));
    assert_eq!(409, epidemic_imper(13, 312, 999, 1, 0.00221, 0.55));
}

#[test]
fn basic_tests_funct() {
    assert_eq!(420, epidemic_funct(18, 432, 1004, 1, 0.00209, 0.51));
    assert_eq!(461, epidemic_funct(12, 288, 1007, 2, 0.00206, 0.45));
    assert_eq!(409, epidemic_funct(13, 312, 999, 1, 0.00221, 0.55));
}
