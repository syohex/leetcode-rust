fn max_pair_strength(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    fn gcd(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        loop {
            let m = a % b;
            if m == 0 {
                return b;
            }

            (a, b) = (b, m);
        }
    }

    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    let mut cache = HashMap::new();
    let mut ret = 0i64;
    for i in 0..len {
        for j in (i + 1)..len {
            let key = (nums[j], nums[i]);
            let m = if let Some(v) = cache.get(&key) {
                *v
            } else {
                let v = gcd(nums[j], nums[i]);
                cache.insert(key, v);
                v
            } as i64;

            let val = (nums[i] as i64 * nums[j] as i64) / (m * m);
            ret = std::cmp::max(ret, val);
        }
    }

    ret
}

fn main() {
    let ret = max_pair_strength(vec![2, 3, 5, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_pair_strength(vec![2, 3, 5]), 15);
    assert_eq!(max_pair_strength(vec![4, 6, 8]), 12);
    assert_eq!(max_pair_strength(vec![3, 3]), 1);
}
