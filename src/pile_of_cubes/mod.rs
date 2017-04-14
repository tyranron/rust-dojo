//! Kata "Build a pile of Cubes":
//! http://www.codewars.com/kata/5592e3bd57b64d00f3000047


/// Returns `n` that satisfies equation `n^3 + (n-1)^3 + ... + 1^3 = m`,
/// or `-1` if such `n` does not exist.
fn find_nb(m: u64) -> i32 {
    let mut r = 0;
    for n in 1u64.. {
        r += n.pow(3);
        if r == m {
            return n as i32;
        }
        if r > m {
            return -1;
        }
    }
    unreachable!();
}


#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn basics_imper_find_nb() {
        assert_eq!(find_nb(4183059834009), 2022);
        assert_eq!(find_nb(24723578342962), -1);
        assert_eq!(find_nb(135440716410000), 4824);
        assert_eq!(find_nb(40539911473216), 3568);
        assert_eq!(find_nb(26825883955641), 3218);
    }
}
