fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    nums[len - 1] + nums[len - 2] - nums[0]
}

fn main() {
    let ret = maximize_expression_of_three(vec![1, 4, 2, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximize_expression_of_three(vec![1, 4, 2, 5]), 8);
    assert_eq!(maximize_expression_of_three(vec![-2, 0, 5, -2, 4]), 11);
}
