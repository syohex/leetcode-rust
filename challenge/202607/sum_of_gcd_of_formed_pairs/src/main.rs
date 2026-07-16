fn gcd_sum(nums: Vec<i32>) -> i64 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }

    use std::collections::HashMap;

    let mut max = nums[0];
    let mut prefix = vec![nums[0]];
    let mut cache = HashMap::new();

    for n in nums.into_iter().skip(1) {
        max = std::cmp::max(max, n);
        let a = std::cmp::max(max, n);
        let b = std::cmp::min(max, n);

        let key = (a, b);
        if let Some(v) = cache.get(&key) {
            prefix.push(*v);
        } else {
            let v = gcd(a, b);
            prefix.push(v);
            cache.insert(key, v);
        }
    }

    prefix.sort_unstable();

    let len = prefix.len();
    let limit = len / 2;
    let mut ret = 0;
    for i in 0..limit {
        let key = (prefix[len - i - 1], prefix[i]);
        if let Some(v) = cache.get(&key) {
            ret += *v as i64;
        } else {
            let v = gcd(prefix[len - i - 1], prefix[i]);
            ret += v as i64;
            cache.insert(key, v);
        }
    }

    ret
}
fn main() {
    let ret = gcd_sum(vec![3, 6, 2, 8]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(gcd_sum(vec![2, 6, 4]), 2);
    assert_eq!(gcd_sum(vec![3, 6, 2, 8]), 5);
}
