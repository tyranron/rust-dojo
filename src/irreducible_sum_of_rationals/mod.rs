//! Kata "Irreducible Sum of Rationals":
//! http://www.codewars.com/kata/5517fcb0236c8826940003c9

use std::cmp::{max, min};

/// Sums given list rationals and returns this sum in irreducible form.
fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    if l.is_empty() {
        None
    } else {
        Some(l.iter().fold((0, 1), |(n1, d1), &(n2, d2)| {
            let (n, d) = (n1 * d2 + n2 * d1, d1 * d2);
            let gcd = euclid_gcd(max(n, d), min(n, d));
            (n / gcd, d / gcd)
        }))
    }
}

/// Finds GCD for `a` and `b` naturals with Euclidean algorithm.
///
/// Note: `a` must be not less than `b`.
fn euclid_gcd(a: i64, b: i64) -> i64 {
    let r = a % b;
    if r == 0 { b } else { euclid_gcd(b, r) }
}


#[cfg(test)]
fn testing(l: Vec<(i64, i64)>, exp: Option<(i64, i64)>) -> () {
    assert_eq!(sum_fracts(l), exp)
}

#[test]
fn tests_sum_fracts() {
    testing(vec![(1, 2), (1, 3), (1, 4)], Some((13, 12)));
    testing(vec![(1, 3), (5, 3)], Some((2, 1)));
    testing(vec![(10, 20), (100, 300), (10, 40)], Some((13, 12)));
}

#[test]
fn test_euclid() {
    assert_eq!(euclid_gcd(13, 1), 1);
    assert_eq!(euclid_gcd(13, 5), 1);
    assert_eq!(euclid_gcd(8, 4), 4);
    assert_eq!(euclid_gcd(9, 6), 3);
}
