fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        left: i32,
        right: i32,
        i: usize,
        nums: &Vec<i32>,
        multipliers: &Vec<i32>,
        cache: &mut HashMap<(i32, i32, usize), i32>,
    ) -> i32 {
        if i >= multipliers.len() {
            return 0;
        }

        if let Some(v) = cache.get(&(left, right, i)) {
            return *v;
        }

        let left_value = f(left + 1, right, i + 1, nums, multipliers, cache)
            + nums[left as usize] * multipliers[i];
        let right_value = f(left, right - 1, i + 1, nums, multipliers, cache)
            + nums[right as usize] * multipliers[i];

        let v = std::cmp::max(left_value, right_value);
        cache.insert((left, right, i), v);
        v
    }

    let mut cache = HashMap::new();
    f(
        0,
        (nums.len() - 1) as i32,
        0,
        &nums,
        &multipliers,
        &mut cache,
    )
}

fn main() {
    let nums = vec![-5, -3, -3, -2, 7, 1];
    let multipliers = vec![-10, -5, 3, 4, 6];
    let ret = maximum_score(nums, multipliers);
    println!("ret={ret}");
}

#[test]
fn test_maxmium_score() {
    {
        let nums = vec![1, 2, 3];
        let multipliers = vec![3, 2, 1];
        let ret = maximum_score(nums, multipliers);
        assert_eq!(ret, 14);
    }
    {
        let nums = vec![-5, -3, -3, -2, 7, 1];
        let multipliers = vec![-10, -5, 3, 4, 6];
        let ret = maximum_score(nums, multipliers);
        assert_eq!(ret, 102);
    }
}
