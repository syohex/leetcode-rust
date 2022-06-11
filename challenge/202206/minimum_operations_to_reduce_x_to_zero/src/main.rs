fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let total: i32 = nums.iter().sum();

    let mut left = 0usize;
    let mut sum = 0;

    let mut ret = -1;
    for i in 0..nums.len() {
        sum += nums[i];

        while sum > total - x && left <= i {
            sum -= nums[left];
            left += 1;
        }

        if sum == total - x {
            ret = std::cmp::max(ret, (i - left + 1) as i32);
        }
    }

    if ret == -1 {
        -1
    } else {
        nums.len() as i32 - ret
    }
}

fn main() {
    let nums = vec![1, 1, 4, 2, 3];
    let ret = min_operations(nums, 5);
    println!("ret={ret}");
}

#[test]
fn test_min_operations() {
    {
        let nums = vec![1, 1, 4, 2, 3];
        let ret = min_operations(nums, 5);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![5, 6, 7, 8, 9];
        let ret = min_operations(nums, 4);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![3, 2, 20, 1, 1, 3];
        let ret = min_operations(nums, 10);
        assert_eq!(ret, 5);
    }
}
