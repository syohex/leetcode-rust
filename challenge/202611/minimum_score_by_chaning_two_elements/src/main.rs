fn minimize_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    let ret1 = nums[len - 1] - nums[2];
    let ret2 = nums[len - 3] - nums[0];
    let ret3 = nums[len - 2] - nums[1];
    std::cmp::min(ret1, std::cmp::min(ret2, ret3))
}

fn main() {
    let ret = minimize_sum(vec![1, 4, 7, 8, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimize_sum(vec![58, 42, 8, 75, 28]), 30);
    assert_eq!(minimize_sum(vec![1, 4, 7, 8, 5]), 3);
    assert_eq!(minimize_sum(vec![1, 4, 3]), 0);
}
