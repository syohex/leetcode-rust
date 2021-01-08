use std::cmp::{max, min};
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        m.insert(num, i);
    }

    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&index) = m.get(&diff) {
            if index != i {
                return vec![min(i, index) as i32, max(i, index) as i32];
            }
        }
    }

    unreachable!("never reach here");
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

fn main() {
    let ans = two_sum(vec![2, 7, 11, 15], 9);
    println!("two_sum([2, 7, 11, 15], 9) == {:?}", ans);
}
