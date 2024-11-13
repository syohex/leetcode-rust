fn count_fair_paris(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    fn f(mut left: i32, mut right: i32, nums: &[i32], val: i32) -> i32 {
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < val {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left
    }

    let mut nums = nums;
    nums.sort_unstable();

    let limit = nums.len() as i32 - 1;
    let mut ret = 0i64;
    for (i, num) in nums.iter().enumerate() {
        let left = f(i as i32 + 1, limit, &nums, lower - num);
        let right = f(i as i32 + 1, limit, &nums, upper - num + 1);

        ret += (right - left) as i64;
    }

    ret
}

fn main() {
    let nums = vec![0, 1, 7, 4, 4, 5];
    let ret = count_fair_paris(nums, 3, 6);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let ret = count_fair_paris(nums, 3, 6);
        assert_eq!(ret, 6i64);
    }
    {
        let nums = vec![1, 7, 9, 2, 5];
        let ret = count_fair_paris(nums, 11, 11);
        assert_eq!(ret, 1i64);
    }
}
