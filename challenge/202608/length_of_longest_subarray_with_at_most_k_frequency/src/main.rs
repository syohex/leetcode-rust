fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    let mut left = 0;
    let mut ret = 0;

    for (i, n) in nums.iter().enumerate() {
        *h.entry(n).or_insert(0) += 1;

        loop {
            let right_freq = *h.get(n).unwrap();
            if right_freq <= k {
                break;
            }

            if let Some(v) = h.get_mut(&nums[left]) {
                *v -= 1;
            }

            left += 1;
        }

        ret = std::cmp::max(ret, i - left + 1);
    }

    ret as i32
}

fn main() {
    let ret = max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
    assert_eq!(max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
    assert_eq!(max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5, 5], 4), 4);
}
