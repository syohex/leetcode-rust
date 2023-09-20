fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    let diff = sum - x;

    let mut left = 0usize;
    let mut v = 0;

    let mut ret = -1;
    for i in 0..nums.len() {
        v += nums[i];

        while v > diff && left <= i {
            v -= nums[left];
            left += 1;
        }

        if v == diff {
            let n = (i - left + 1) as i32;
            ret = std::cmp::max(ret, n);
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
