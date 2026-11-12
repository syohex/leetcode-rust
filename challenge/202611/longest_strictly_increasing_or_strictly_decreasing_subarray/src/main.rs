fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut max_len = i32::MIN;
    let mut len = 1;

    // check increasing
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            len += 1;
        } else {
            max_len = std::cmp::max(max_len, len);
            len = 1;
        }
    }

    max_len = std::cmp::max(max_len, len);
    len = 1;

    // check decreasing
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            len += 1;
        } else {
            max_len = std::cmp::max(max_len, len);
            len = 1;
        }
    }

    std::cmp::max(max_len, len)
}

fn main() {
    let ret = longest_monotonic_subarray(vec![1, 4, 3, 3, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);
}
