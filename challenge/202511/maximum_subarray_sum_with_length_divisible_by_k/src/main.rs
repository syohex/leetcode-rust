fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut prefixes = vec![0i64];
    let mut sum = 0i64;
    for num in nums {
        sum += num as i64;
        prefixes.push(sum);
    }

    let k = k as i64;
    let mut ret = i64::MIN;
    for i in 1..=len {
        let mut j = i as i64 - k;
        while j >= 0 {
            ret = std::cmp::max(ret, prefixes[i] - prefixes[j as usize]);
            j -= k;
        }
    }

    ret
}

fn main() {
    let nums = vec![-5, 1, 2, -3, 4];
    let ret = max_subarray_sum(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2];
        let ret = max_subarray_sum(nums, 1);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![-1, -2, -3, -4, -5];
        let ret = max_subarray_sum(nums, 4);
        assert_eq!(ret, -10);
    }
    {
        let nums = vec![-5, 1, 2, -3, 4];
        let ret = max_subarray_sum(nums, 2);
        assert_eq!(ret, 4);
    }
}
