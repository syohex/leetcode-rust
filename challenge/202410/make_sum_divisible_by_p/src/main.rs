fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    use std::collections::HashMap;

    let len = nums.len() as i32;
    let mut total = 0;
    for &n in &nums {
        total = (total + n) % p;
    }

    if total == 0 {
        return 0;
    }

    let m = total % p;

    let mut h: HashMap<i32, i32> = HashMap::new();
    h.insert(0, -1);

    let mut ret = len;
    let mut sum = 0;

    for (i, n) in nums.into_iter().enumerate() {
        sum = (sum + n) % p;

        let diff = (p + sum - m) % p;
        if let Some(&v) = h.get(&diff) {
            ret = std::cmp::min(ret, i as i32 - v);
        }

        h.insert(sum, i as i32);
    }

    if ret == len {
        -1
    } else {
        ret
    }
}

fn main() {
    let nums = vec![6, 3, 5, 2];
    let ret = min_subarray(nums, 9);
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
