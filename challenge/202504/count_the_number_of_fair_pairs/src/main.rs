fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    fn lower_bound(mut left: i32, mut right: i32, nums: &[i32], v: i32) -> i32 {
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < v {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left
    }

    let limit = nums.len() as i32 - 1;
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = 0;
    for (i, &n) in nums.iter().enumerate() {
        let start = i as i32 + 1;
        let left = lower_bound(start, limit, &nums, lower - n);
        let right = lower_bound(start, limit, &nums, upper - n + 1);

        ret += (right - left) as i64;
    }

    ret
}

fn main() {
    let nums = vec![0, 1, 7, 4, 4, 5];
    let ret = count_fair_pairs(nums, 3, 6);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let ret = count_fair_pairs(nums, 3, 6);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 7, 9, 2, 5];
        let ret = count_fair_pairs(nums, 11, 11);
        assert_eq!(ret, 1);
    }
}
