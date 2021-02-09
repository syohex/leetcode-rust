use std::cmp::min;
use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut h1: HashMap<i32, i32> = HashMap::new();
    let mut h2: HashMap<i32, i32> = HashMap::new();

    for &n in &nums1 {
        *h1.entry(n).or_insert(0) += 1;
    }
    for &n in &nums2 {
        *h2.entry(n).or_insert(0) += 1;
    }

    let mut ret = vec![];
    for (&k, &v1) in h1.iter() {
        if let Some(&v2) = h2.get(&k) {
            for _ in 0..min(v1, v2) {
                ret.push(k);
            }
        }
    }

    ret
}

fn main() {
    println!("example1={:?}", intersect(vec![1, 2, 2, 1], vec![2, 2]));
}

#[test]
fn test_intersect() {
    {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let ret = intersect(nums1, nums2);
        assert_eq!(ret, vec![2, 2]);
    }
    {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let ret = intersect(nums1, nums2);
        assert_eq!(ret.len(), 2);
        assert!((ret[0] == 4 && ret[1] == 9) || (ret[0] == 9 || ret[1] == 4));
    }
}
