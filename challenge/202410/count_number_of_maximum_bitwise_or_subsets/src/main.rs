fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        current_or: i32,
        max_or: i32,
        nums: &Vec<i32>,
        cache: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if pos == nums.len() {
            if current_or == max_or {
                return 1;
            }

            return 0;
        }

        let key = (pos, current_or);
        if let Some(&v) = cache.get(&key) {
            return v;
        }

        let ret_not_or = f(pos + 1, current_or, max_or, nums, cache);
        let ret_or = f(pos + 1, current_or | nums[pos], max_or, nums, cache);

        let ret = ret_not_or + ret_or;
        cache.insert(key, ret);
        ret
    }

    let max_or = nums.iter().fold(0, |acc, &n| acc | n);
    let mut cache = HashMap::new();
    f(0, 0, max_or, &nums, &mut cache)
}

fn main() {
    let nums = vec![2, 2, 2];
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
