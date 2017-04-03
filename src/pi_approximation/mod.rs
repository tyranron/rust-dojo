//! Kata "PI approximation":
//! http://www.codewars.com/kata/550527b108b86f700000073f

use std::f64::consts::PI;


/// Rounds float to 10 decimals.
fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}

/// Counts iterations count and approximate value of PI
/// by [Leibniz formula][1] with given `epsilon` approximation error.
///
/// [1]: https://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80
fn iter_pi(epsilon: f64) -> (i32, f64) {
    let (mut result, mut sign, mut iters) = (1.0, 1.0, 0);
    while (PI - 4.0 * result).abs() >= epsilon {
        iters += 1;
        sign *= -1.0;
        result += sign / (1 + 2 * iters) as f64;
    }
    (iters + 1, rnd10(4.0 * result))
}


#[cfg(test)]
fn testing(epsilon: f64, exp: (i32, f64)) -> () {
    assert_eq!(iter_pi(epsilon), exp)
}

#[test]
fn tests_iter_pi() {
    testing(0.1, (10, 3.0418396189));
    testing(0.01, (100, 3.1315929036));
    testing(0.001, (1000, 3.1405926538));
    testing(0.0001, (10000, 3.1414926536));
    testing(0.00001, (100001, 3.1416026535));
    testing(0.000001, (1000001, 3.1415936536));
    testing(0.05, (20, 3.0916238067));
}
