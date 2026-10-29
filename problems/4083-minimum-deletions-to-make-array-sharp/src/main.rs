fn min_deletions(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let ret = min_deletions(vec![1, 1, 2, 2, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_deletions(vec![1, 1, 2, 2, 2, 3]), 3);
    assert_eq!(min_deletions(vec![4, 4, 9, 4]), 1);
}
