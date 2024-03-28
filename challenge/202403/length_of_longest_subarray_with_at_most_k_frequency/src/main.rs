fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut left = 0;
    let mut right = 0;

    let mut ret = 0;
    let mut h = HashMap::new();
    while left < nums.len() && right < nums.len() {
        *h.entry(nums[right]).or_insert(0) += 1;

        loop {
            let v = *h.get(&nums[right]).unwrap();
            if v <= k {
                break;
            }

            if let Some(lv) = h.get_mut(&nums[left]) {
                *lv -= 1;
            }

            left += 1;
        }

        ret = std::cmp::max(ret, right - left + 1);
        right += 1;
    }

    ret as i32
}

fn main() {
    let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
    let ret = max_subarray_length(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let ret = max_subarray_length(nums, 2);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let ret = max_subarray_length(nums, 1);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![5, 5, 5, 5, 5, 5, 5, 5];
        let ret = max_subarray_length(nums, 4);
        assert_eq!(ret, 4);
    }
}
