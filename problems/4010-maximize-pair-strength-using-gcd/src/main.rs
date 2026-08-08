fn max_pair_strength(nums: Vec<i32>) -> i64 {
    use std::collections::HashSet;

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
    nums.sort_unstable_by_key(|n| std::cmp::Reverse(*n));
    let len = nums.len();

    let mut ret = 0;
    for i in 0..len {
        let mut checked = HashSet::new();
        for j in (i + 1)..len {
            let m = gcd(nums[i], nums[j]) as i64;
            if checked.contains(&m) {
                continue;
            }

            let v = (nums[i] as i64 * nums[j] as i64) / (m * m);
            ret = std::cmp::max(ret, v);
            checked.insert(m);
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
    assert_eq!(max_pair_strength(vec![1, 24, 9, 8]), 72);
    assert_eq!(max_pair_strength(vec![7, 18, 12]), 126);
    assert_eq!(max_pair_strength(vec![2, 3, 5]), 15);
    assert_eq!(max_pair_strength(vec![4, 6, 8]), 12);
    assert_eq!(max_pair_strength(vec![3, 3]), 1);
}
