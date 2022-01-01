fn dp(nums: &Vec<i32>, left: usize, right: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
    if left > right {
        return 0;
    }

    if cache[left][right] != -1 {
        return cache[left][right];
    }

    let mut ret = 0;
    for i in left..=right {
        let val = nums[left - 1] * nums[i] * nums[right + 1];
        let a = dp(nums, left, i - 1, cache) + dp(nums, i + 1, right, cache);
        ret = std::cmp::max(ret, val + a);
    }

    cache[left][right] = ret;
    ret
}

fn max_coins(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut v = vec![0; len + 2];
    v[0] = 1;
    v[len + 1] = 1;
    for i in 0..len {
        v[i + 1] = nums[i];
    }

    let mut cache = vec![vec![-1; len + 2]; len + 2];
    dp(&v, 1, len, &mut cache)
}

fn main() {
    let nums = vec![3, 1, 5, 8];
    println!("ret={}", max_coins(nums));
}

#[test]
fn test_max_coins() {
    {
        let nums = vec![3, 1, 5, 8];
        assert_eq!(max_coins(nums), 167);
    }
    {
        let nums = vec![1, 5];
        assert_eq!(max_coins(nums), 10);
    }
}
