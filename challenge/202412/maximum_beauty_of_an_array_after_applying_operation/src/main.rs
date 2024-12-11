fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let max_diff = 2 * k;
    let mut left = 0;
    let mut ret = 0;
    for right in 0..nums.len() {
        while left <= right && nums[right] - nums[left] > max_diff {
            left += 1;
        }

        ret = std::cmp::max(ret, right - left + 1);
    }

    ret as i32
}

fn main() {
    let nums = vec![4, 6, 1, 2];
    let ret = maximum_beauty(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 6, 1, 2];
        let ret = maximum_beauty(nums, 2);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 1, 1, 1];
        let ret = maximum_beauty(nums, 10);
        assert_eq!(ret, 4);
    }
}
