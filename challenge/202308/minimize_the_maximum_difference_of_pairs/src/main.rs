fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
    fn f(nums: &Vec<i32>, threshold: i32) -> i32 {
        let mut ret = 0;
        let mut i = 1;
        while i < nums.len() {
            if nums[i] - nums[i - 1] <= threshold {
                i += 1;
                ret += 1;
            }

            i += 1;
        }

        ret
    }

    let mut nums = nums;
    nums.sort_unstable();

    let mut left = 0;
    let mut right = (nums.last().unwrap() - nums[0]).abs();

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
fn test_minimize_max() {
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
