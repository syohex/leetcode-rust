fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max = *nums.iter().max().unwrap();
    let mut left = 0;
    let mut count = 0;
    let mut ret = 0;

    for (right, &n) in nums.iter().enumerate() {
        if max == n {
            count += 1;
        }

        while left <= right && count >= k {
            if nums[left] == max {
                count -= 1;
            }
            left += 1;
        }

        ret += left as i64;
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 2, 3, 3];
    let ret = count_subarrays(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 2, 3, 3];
        let ret = count_subarrays(nums, 2);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 4, 2, 1];
        let ret = count_subarrays(nums, 3);
        assert_eq!(ret, 0);
    }
}
