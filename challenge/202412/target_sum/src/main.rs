fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        nums: &Vec<i32>,
        target: i32,
        acc: i32,
        cache: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if pos == nums.len() {
            if acc == target {
                return 1;
            }
            return 0;
        }

        let key = (pos, acc);
        if let Some(v) = cache.get(&key) {
            *v
        } else {
            let ret1 = f(pos + 1, nums, target, acc + nums[pos], cache);
            let ret2 = f(pos + 1, nums, target, acc - nums[pos], cache);
            let ret = ret1 + ret2;
            cache.insert(key, ret);
            ret
        }
    }

    let mut cache = HashMap::new();
    f(0, &nums, target, 0, &mut cache)
}

fn main() {
    let nums = vec![
        0, 35, 32, 3, 4, 16, 12, 25, 47, 9, 14, 29, 7, 26, 17, 42, 21, 23, 48, 18,
    ];
    let ret = find_target_sum_ways(nums, 20);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 1, 1, 1, 1];
        let ret = find_target_sum_ways(nums, 3);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1];
        let ret = find_target_sum_ways(nums, 1);
        assert_eq!(ret, 1);
    }
}
