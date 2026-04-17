fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn reverse(n: i32) -> i32 {
        let mut n = n;
        let mut v = 0;
        while n > 0 {
            let m = n % 10;
            v = 10 * v + m;
            n /= 10;
        }

        v
    }

    let mut h = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        h.entry(*n).or_insert(vec![]).push(i);
    }

    let mut ret = usize::MAX;
    for (i, n) in nums.into_iter().enumerate() {
        let rev = reverse(n);
        if let Some(v) = h.get(&rev) {
            let pos = v.partition_point(|m| *m <= i);
            if pos != v.len() {
                ret = std::cmp::min(ret, v[pos] - i);
            }
        }
    }

    if ret == usize::MAX { -1 } else { ret as i32 }
}

fn main() {
    let ret = min_mirror_pair_distance(vec![12, 21, 45, 33, 54]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_mirror_pair_distance(vec![12, 21, 45, 33, 54]), 1);
    assert_eq!(min_mirror_pair_distance(vec![120, 21]), 1);
    assert_eq!(min_mirror_pair_distance(vec![21, 120]), -1);
}
