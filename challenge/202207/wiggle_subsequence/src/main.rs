fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        nums: &Vec<i32>,
        prev: i32,
        is_up: bool,
        cache: &mut HashMap<(usize, i32, bool), i32>,
    ) -> i32 {
        if pos == nums.len() {
            return 0;
        }

        let key = &(pos, prev, is_up);
        if let Some(&v) = cache.get(key) {
            return v;
        }

        let diff = nums[pos] - prev;
        let mut ret1 = 0;
        if (is_up && diff < 0) || (!is_up && diff > 0) {
            ret1 = 1 + f(pos + 1, nums, nums[pos], !is_up, cache);
        }

        let ret2 = f(pos + 1, nums, prev, is_up, cache);
        let ret = std::cmp::max(ret1, ret2);
        cache.insert((pos, prev, is_up), ret);

        ret
    }

    let mut cache = HashMap::new();
    let ret1 = f(0, &nums, 1001, true, &mut cache);
    let ret2 = f(0, &nums, -1001, false, &mut cache);

    std::cmp::max(ret1, ret2)
}

fn main() {
    let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    let ret = wiggle_max_length(nums);
    println!("ret={ret}");
}

#[test]
fn test_wiggle_max_length() {
    {
        let nums = vec![1, 7, 4, 9, 2, 5];
        let ret = wiggle_max_length(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        let ret = wiggle_max_length(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let ret = wiggle_max_length(nums);
        assert_eq!(ret, 2);
    }
}
