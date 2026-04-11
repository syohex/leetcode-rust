fn minimum_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        h.entry(n).or_insert(vec![]).push(i);
    }

    let mut min_val = usize::MAX;
    for v in h.into_values().filter(|v| v.len() >= 3) {
        for i in 2..v.len() {
            min_val = std::cmp::min(min_val, 2 * (v[i] - v[i - 2]));
        }
    }

    if min_val == usize::MAX {
        -1
    } else {
        min_val as i32
    }
}

fn main() {
    let ret = minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_distance(vec![1, 1, 1, 1]), 4);
    assert_eq!(minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    assert_eq!(minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    assert_eq!(minimum_distance(vec![-1]), -1);
}
