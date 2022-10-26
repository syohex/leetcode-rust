fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let mut acc = vec![0; len + 1];

    for i in 0..len {
        acc[i + 1] = acc[i] + nums[i];
    }

    for i in 2..=len {
        for j in 0..=(i - 2) {
            let sum = acc[i] - acc[j];
            if sum % k == 0 {
                return true;
            }
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
    // {
    //     let nums = vec![23, 2, 4, 6, 7];
    //     let ret = check_subarray_sum(nums, 6);
    //     assert!(ret);
    // }
    // {
    //     let nums = vec![23, 2, 6, 4, 7];
    //     let ret = check_subarray_sum(nums, 6);
    //     assert!(ret);
    // }
    // {
    //     let nums = vec![23, 2, 6, 4, 7];
    //     let ret = check_subarray_sum(nums, 13);
    //     assert!(!ret);
    // }
    {
        let nums = vec![0, 0];
        let ret = check_subarray_sum(nums, 1);
        assert!(ret);
    }
}
