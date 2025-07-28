fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        nums: &[i32],
        ors: i32,
        max: i32,
        cache: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if pos == nums.len() {
            if ors == max {
                return 1;
            }

            return 0;
        }

        let key = (pos, ors);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let ret1 = f(pos + 1, nums, ors | nums[pos], max, cache);
        let ret2 = f(pos + 1, nums, ors, max, cache);
        let ret = ret1 + ret2;

        cache.insert(key, ret);
        ret
    }

    let max = nums.iter().fold(0, |acc, &n| acc | n);
    let mut cache = HashMap::new();
    f(0, &nums, 0, max, &mut cache)
}
fn main() {
    let nums = vec![3, 1];
    let ret = count_max_or_subsets(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 1];
        let ret = count_max_or_subsets(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 2, 2];
        let ret = count_max_or_subsets(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![3, 2, 1, 5];
        let ret = count_max_or_subsets(nums);
        assert_eq!(ret, 6);
    }
}
