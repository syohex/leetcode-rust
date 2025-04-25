fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    use std::collections::HashMap;

    let mut prefix = 0;
    let mut ret = 0;
    let mut h = HashMap::new();
    h.insert(0, 1);
    for n in nums {
        prefix += if n % modulo == k { 1 } else { 0 };
        let key = (prefix + modulo - k) % modulo;
        if let Some(&v) = h.get(&key) {
            ret += v as i64;
        }
        *h.entry(prefix % modulo).or_insert(0) += 1;
    }

    ret
}

fn main() {
    let nums = vec![3, 2, 4];
    let ret = count_interesting_subarrays(nums, 2, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 2, 4];
        let ret = count_interesting_subarrays(nums, 2, 1);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![3, 1, 9, 6];
        let ret = count_interesting_subarrays(nums, 3, 0);
        assert_eq!(ret, 2);
    }
}
