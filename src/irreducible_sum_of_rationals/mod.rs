//! Kata "Irreducible Sum of Rationals":
//! http://www.codewars.com/kata/5517fcb0236c8826940003c9


fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    // your code
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
