fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let mut h: HashMap<i32, usize> = HashMap::new();
    h.insert(0, 0);

    let mut sum = 0;

    for (i, num) in nums.into_iter().enumerate() {
        sum += num;

        let m = sum % k;
        if let Some(j) = h.get(&m) {
            if *j < i {
                return true;
            }
        } else {
            h.insert(m, i + 1);
        }
    }

    false
}

fn main() {
    let nums = vec![23, 2, 4, 6, 7];
    let ret = check_subarray_sum(nums, 6);
    println!("ret={ret}");
}

#[test]
fn test_check_subarray_sum() {
    {
        let nums = vec![23, 2, 4, 6, 7];
        let ret = check_subarray_sum(nums, 6);
        assert!(ret);
    }
    {
        let nums = vec![23, 2, 6, 4, 7];
        let ret = check_subarray_sum(nums, 6);
        assert!(ret);
    }
    {
        let nums = vec![23, 2, 6, 4, 7];
        let ret = check_subarray_sum(nums, 13);
        assert!(!ret);
    }
    {
        let nums = vec![0, 0];
        let ret = check_subarray_sum(nums, 1);
        assert!(ret);
    }
}
