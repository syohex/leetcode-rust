fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
    fn f(nums: &[i32], p: i32) -> i32 {
        let mut pairs = 0;
        let mut i = 1;
        while i < nums.len() {
            if (nums[i] - nums[i - 1]).abs() <= p {
                pairs += 1;
                i += 2;
            } else {
                i += 1;

            }
        }

        pairs
    }

    let mut nums = nums;
    nums.sort_unstable();

    let mut left = 0;
    let mut right = (nums[nums.len() - 1] - nums[0]).abs();
    while left < right {
        let mid = left + (right - left) / 2;
        if f(&nums, mid) >= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    let nums = vec![10, 1, 2, 7, 1, 3];
    let ret = minimize_max(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let ret = minimize_max(nums, 2);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![4, 2, 1, 2];
        let ret = minimize_max(nums, 1);
        assert_eq!(ret, 0);
    }
}
