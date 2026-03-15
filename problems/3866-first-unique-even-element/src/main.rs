fn first_unique_even(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut min_idx = nums.len();
    let mut h = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        if n % 2 == 0 {
            h.entry(n).or_insert(vec![]).push(i);
        }
    }

    let mut ret = -1;
    for (k, v) in h {
        if v.len() == 1 && v[0] < min_idx {
            min_idx = v[0];
            ret = k;
        }
    }

    ret
}

fn main() {
    let ret = first_unique_even(vec![3, 4, 2, 5, 6, 4, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(first_unique_even(vec![3, 4, 2, 5, 4, 6]), 2);
    assert_eq!(first_unique_even(vec![4, 4]), -1);
}
