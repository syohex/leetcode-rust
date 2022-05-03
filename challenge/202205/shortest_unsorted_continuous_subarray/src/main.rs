fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut tmp = nums.to_owned();
    tmp.sort_unstable();

    let mut left = nums.len();
    let mut right = 0;

    for i in 0..nums.len() {
        if nums[i] != tmp[i] {
            left = std::cmp::min(left, i);
            right = std::cmp::max(right, i);
        }
    }

    if left == nums.len() {
        0
    } else {
        (right - left + 1) as i32
    }
}

fn main() {
    let nums = vec![2, 6, 4, 8, 10, 9, 15];
    let ret = find_unsorted_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_unsorted_subarray() {
    {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        let ret = find_unsorted_subarray(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = find_unsorted_subarray(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![0];
        let ret = find_unsorted_subarray(nums);
        assert_eq!(ret, 0);
    }
}
