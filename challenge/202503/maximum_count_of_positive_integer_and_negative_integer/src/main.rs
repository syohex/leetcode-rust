fn maximum_count(nums: Vec<i32>) -> i32 {
    fn lower_bound(nums: &[i32], v: i32) -> usize {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < v {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left as usize
    }

    let zero_bound = lower_bound(&nums, 0);
    let one_bound = lower_bound(&nums, 1);
    std::cmp::max(zero_bound, nums.len() - one_bound) as i32
}

fn main() {
    let nums = vec![-3, -2, -1, 0, 0, 1, 2];
    let ret = maximum_count(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![-2, -1, -1, 1, 2, 3];
        let ret = maximum_count(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![-3, -2, -1, 0, 0, 1, 2];
        let ret = maximum_count(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = maximum_count(nums);
        assert_eq!(ret, 4);
    }
}
