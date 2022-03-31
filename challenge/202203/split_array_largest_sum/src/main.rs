fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    use std::cmp::{max, min};
    use std::collections::HashMap;

    fn f(
        accs: &[i32],
        cur: usize,
        subarrays: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(v) = cache.get(&(cur, subarrays)) {
            return *v;
        }

        let len = accs.len() - 1;
        if subarrays == 1 {
            let v = accs[len] - accs[cur];
            cache.insert((cur, subarrays), v);
            return v;
        }

        let mut ret = std::i32::MAX;
        for i in cur..=(len - subarrays) {
            let prev_part = accs[i + 1] - accs[cur];
            let largest = max(prev_part, f(accs, i + 1, subarrays - 1, cache));
            ret = min(ret, largest);

            if prev_part >= largest {
                break;
            }
        }

        cache.insert((cur, subarrays), ret);
        ret
    }

    let mut accs = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        accs[i + 1] = accs[i] + nums[i];
    }

    let mut cache = HashMap::new();
    f(&accs, 0, m as usize, &mut cache)
}

fn main() {
    let nums = vec![7, 2, 5, 10, 8];
    let ret = split_array(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test_split_array() {
    {
        let nums = vec![7, 2, 5, 10, 8];
        assert_eq!(split_array(nums, 2), 18);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(split_array(nums, 2), 9);
    }
    {
        let nums = vec![1, 4, 4];
        assert_eq!(split_array(nums, 3), 4);
    }
}
