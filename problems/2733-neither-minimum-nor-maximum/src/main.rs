fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    if nums.len() <= 2 {
        -1
    } else {
        nums[1]
    }
}

fn main() {
    let nums = vec![3, 2, 1, 4];
    let ret = find_non_min_or_max(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_non_min_or_max() {
    {
        let nums = vec![3, 2, 1, 4];
        let ret = find_non_min_or_max(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 2];
        let ret = find_non_min_or_max(nums);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![2, 1, 3];
        let ret = find_non_min_or_max(nums);
        assert_eq!(ret, 2);
    }
}
