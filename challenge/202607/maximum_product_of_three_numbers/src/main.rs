fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    std::cmp::max(
        nums[0] * nums[1] * nums[len - 1],
        nums[len - 3] * nums[len - 2] * nums[len - 1],
    )
}

fn main() {
    let ret = maximum_product(vec![-100, -2, -3, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(maximum_product(vec![-1, -2, -3]), -6);
    assert_eq!(maximum_product(vec![-100, -2, -3, 1]), 300);
    assert_eq!(
        maximum_product(vec![73, 96, 97, 91, 60, 52, 53, 60, 8, 80]),
        847392
    );
}
