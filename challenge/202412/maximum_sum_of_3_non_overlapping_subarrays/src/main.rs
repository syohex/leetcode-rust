fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        count: usize,
        k: usize,
        sums: &Vec<i32>,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if count == 0 {
            return 0;
        }
        if pos >= sums.len() {
            return -1_000_000_000;
        }

        if let Some(v) = cache.get(&(pos, count)) {
            return *v;
        }

        let ret1 = sums[pos] + f(pos + k, count - 1, k, sums, cache);
        let ret2 = f(pos + 1, count, k, sums, cache);
        let ret = std::cmp::max(ret1, ret2);
        cache.insert((pos, count), ret);

        ret
    }

    fn g(
        pos: usize,
        count: usize,
        k: usize,
        sums: &Vec<i32>,
        ret: &mut Vec<i32>,
        cache: &mut HashMap<(usize, usize), i32>,
    ) {
        if count == 0 || pos >= sums.len() {
            return;
        }

        let ret1 = sums[pos] + f(pos + k, count - 1, k, sums, cache);
        let ret2 = f(pos + 1, count, k, sums, cache);
        if ret1 >= ret2 {
            ret.push(pos as i32);
            g(pos + k, count - 1, k, sums, ret, cache);
        } else {
            g(pos + 1, count, k, sums, ret, cache);
        }
    }

    let k = k as usize;
    let mut sum = 0;
    for i in 0..k {
        sum += nums[i];
    }

    let mut sums = vec![sum];
    for i in k..nums.len() {
        sum = sum - nums[i - k] + nums[i];
        sums.push(sum);
    }

    let mut cache = HashMap::new();
    f(0, 3, k, &sums, &mut cache);
    let mut ret = vec![];
    g(0, 3, k, &sums, &mut ret, &mut cache);

    ret
}

fn main() {
    let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
    let ret = max_sum_of_three_subarrays(nums, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let expected = vec![0, 3, 5];
        let ret = max_sum_of_three_subarrays(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        let expected = vec![0, 2, 4];
        let ret = max_sum_of_three_subarrays(nums, 2);
        assert_eq!(ret, expected);
    }
}
