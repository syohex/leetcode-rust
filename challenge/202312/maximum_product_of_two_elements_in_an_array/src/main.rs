fn max_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    nums.sort_unstable();
    (nums[len - 1] - 1) * (nums[len - 2] - 1)
}

fn main() {
    let nums = vec![3, 4, 5, 2];
    let ret = max_product(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 4, 5, 2];
        let ret = max_product(nums);
        assert_eq!(ret, 12);
    }
    {
        let nums = vec![1, 5, 4, 5];
        let ret = max_product(nums);
        assert_eq!(ret, 16);
    }
    {
        let nums = vec![3, 7];
        let ret = max_product(nums);
        assert_eq!(ret, 12);
    }
}
