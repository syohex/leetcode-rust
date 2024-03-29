fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max = nums.iter().max().unwrap();

    let mut ret = 0i64;
    let mut left = 0;
    let mut right = 0;

    let mut count = 0;
    while right < nums.len() {
        if nums[right] == *max {
            count += 1;
        }

        while count >= k && left < nums.len() {
            if nums[left] == *max {
                count -= 1;
            }

            left += 1;
        }

        ret += left as i64;
        right += 1;
    }

    ret
}

fn main() {
    let nums = vec![4, 3, 7, 10, 2, 10, 1, 6, 10, 7, 10, 10, 9, 8, 3];
    let ret = count_subarrays(nums, 3);
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
    {
        let nums = vec![4, 3, 7, 10, 2, 10, 1, 6, 10, 7, 10, 10, 9, 8, 3];
        let ret = count_subarrays(nums, 3);
        assert_eq!(ret, 50);
    }
}
