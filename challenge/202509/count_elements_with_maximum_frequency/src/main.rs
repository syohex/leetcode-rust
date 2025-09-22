fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let h = nums.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });
    let max_freq = h.values().fold(0, |acc, &n| std::cmp::max(acc, n));

    h.values().filter(|&n| *n == max_freq).sum()
}

fn main() {
    let ret = max_frequency_elements(vec![1, 2, 2, 3, 1, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    assert_eq!(max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
}
