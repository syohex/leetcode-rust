fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;
    let nums: Vec<i64> = nums.into_iter().map(|n| n as i64).collect();

    let mut h = HashMap::new();
    let k = k as usize;

    let mut sum = 0;
    for &num in nums.iter().take(k) {
        *h.entry(num).or_insert(0) += 1;
        sum += num;
    }

    let mut ret = if h.len() == k { sum } else { 0 };
    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        *h.entry(nums[i]).or_insert(0) += 1;

        let v = *h.get(&nums[i - k]).unwrap();
        if v == 1 {
            h.remove(&nums[i - k]);
        } else {
            h.insert(nums[i - k], v - 1);
        }

        if h.len() == k {
            ret = std::cmp::max(ret, sum);
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 5, 4, 2, 9, 9, 9];
    let ret = maximum_subarray_sum(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let ret = maximum_subarray_sum(nums, 3);
        assert_eq!(ret, 15);
    }
    {
        let nums = vec![4, 4, 4];
        let ret = maximum_subarray_sum(nums, 3);
        assert_eq!(ret, 0);
    }
}
