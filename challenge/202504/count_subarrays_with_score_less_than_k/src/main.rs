fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let nums: Vec<i64> = nums.into_iter().map(|n| n as i64).collect();
    let mut left = 0;
    let mut sum = 0i64;

    let mut ret = 0;
    for right in 0..nums.len() {
        sum += nums[right];
        while left <= right && sum * ((right - left + 1) as i64) >= k {
            sum -= nums[left];
            left += 1;
        }

        if left <= right {
            ret += (right - left + 1) as i64;
        }
    }

    ret
}

fn main() {
    let nums = vec![2, 1, 4, 3, 5];
    let ret = count_subarrays(nums, 10);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 4, 3, 5];
        let ret = count_subarrays(nums, 10);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 1, 1];
        let ret = count_subarrays(nums, 5);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![9, 5, 3, 8, 4, 7, 2, 7, 4, 5, 4, 9, 1, 4, 8, 10, 8, 10, 4, 7];
        let ret = count_subarrays(nums, 4);
        assert_eq!(ret, 3);
    }
}
