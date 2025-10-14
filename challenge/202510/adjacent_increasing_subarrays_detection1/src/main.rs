fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let k = k as usize;

    for i in 0..=(len - 2 * k) {
        let mut ok = true;
        for j in 1..k {
            if nums[i + j - 1] >= nums[i + j] || nums[i + k + j - 1] >= nums[i + k + j] {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

fn main() {
    let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
    let ret = has_increasing_subarrays(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![-15, 19];
        let ret = has_increasing_subarrays(nums, 1);
        assert!(ret);
    }
    {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let ret = has_increasing_subarrays(nums, 3);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let ret = has_increasing_subarrays(nums, 5);
        assert!(!ret);
    }
}
