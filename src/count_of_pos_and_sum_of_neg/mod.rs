//! Kata "Count of positives / sum of negatives":
//! http://www.codewars.com/kata/576bb71bbbcf0951d5000044


/// For given vec counts number of its positive items and sum of its negative
/// items. The result is returned as two-items vec.
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { return input }
    let (count, num) = input.iter()
        .fold((0, 0), |(count, num), &v| if v > 0 {
            (count + 1, num)
        } else {
            (count, num + v)
        });
    vec![count, num]
}


#[test]
fn returns_expected() {
    let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14,
                          -15];
    let expected1 = vec![10, -65];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}

#[test]
fn empty_on_empty() {
    assert_eq!(count_positives_sum_negatives(Vec::new()), Vec::new());
}
