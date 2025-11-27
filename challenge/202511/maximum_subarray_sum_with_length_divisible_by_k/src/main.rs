fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut min_sums = vec![i64::MAX / 2; k as usize];
    let mut sum = 0i64;
    let mut ret = i64::MIN;

    min_sums[k as usize - 1] = 0;
    for (i, num) in nums.into_iter().enumerate() {
        sum += num as i64;

        let m = i % k as usize;
        ret = std::cmp::max(ret, sum - min_sums[m]);
        min_sums[m] = std::cmp::min(sum, min_sums[m]);
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
