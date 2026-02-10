fn longest_balanced(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let len = nums.len();

    let mut ret = 0;
    for i in 0..len {
        let mut odds = HashSet::new();
        let mut evens = HashSet::new();
        for j in i..len {
            if nums[j] % 2 == 0 {
                evens.insert(nums[j]);
            } else {
                odds.insert(nums[j]);
            }

            if evens.len() == odds.len() {
                ret = std::cmp::max(ret, j - i + 1);
            }
        }
    }

    ret as i32
}

fn main() {
    let ret = longest_balanced(vec![2, 5, 4, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_balanced(vec![2, 5, 4, 3]), 4);
    assert_eq!(longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    assert_eq!(longest_balanced(vec![1, 2, 3, 2]), 3);
}
