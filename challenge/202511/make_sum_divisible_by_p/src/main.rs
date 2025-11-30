fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    use std::collections::HashMap;

    let total_mod = nums.iter().fold(0, |acc, &n| (acc + n) % p);
    if total_mod == 0 {
        return 0;
    }

    let mut h = HashMap::new();
    h.insert(0, -1);

    let len = nums.len() as i32;
    let mut sum = 0;
    let mut ret = len;
    for (i, n) in nums.into_iter().enumerate() {
        sum = (sum + n) % p;
        let diff = (sum + p - total_mod) % p;
        if let Some(&pos) = h.get(&diff) {
            ret = std::cmp::min(ret, i as i32 - pos);
        }
        h.insert(sum, i as i32);
    }

    if ret == len as i32 {
        -1
    } else {
        ret
    }
}

fn main() {
    let nums = vec![3, 1, 4, 2];
    let ret = min_subarray(nums, 6);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 1, 4, 2];
        let ret = min_subarray(nums, 6);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![6, 3, 5, 2];
        let ret = min_subarray(nums, 9);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = min_subarray(nums, 3);
        assert_eq!(ret, 0);
    }
}
