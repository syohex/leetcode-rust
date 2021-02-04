use std::cmp::max;
use std::collections::HashMap;

fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        *h.entry(n).or_insert(0) += 1;
    }

    let mut ret = 0;
    for (&k, &count) in &h {
        if let Some(&count1) = h.get(&(k + 1)) {
            ret = max(ret, count + count1);
        }
    }

    ret
}

fn main() {
    println!("answer={}", find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
}

#[test]
fn test_find_lfs() {
    assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(find_lhs(vec![1, 1, 1, 1]), 0);
}
