fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    (nums[len - 1] * nums[len - 2]) - (nums[1] * nums[0])
}

fn main() {
    let nums = vec![5, 6, 2, 7, 4];
    let ret = max_product_difference(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 6, 2, 7, 4];
        let ret = max_product_difference(nums);
        assert_eq!(ret, 34);
    }
    {
        let nums = vec![4, 2, 5, 9, 7, 4, 8];
        let ret = max_product_difference(nums);
        assert_eq!(ret, 64);
    }
}
