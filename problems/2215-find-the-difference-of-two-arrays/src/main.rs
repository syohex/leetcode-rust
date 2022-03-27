fn find_differenct(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    let h1: HashSet<i32> = nums1.into_iter().collect();
    let h2: HashSet<i32> = nums2.into_iter().collect();

    let mut v1 = vec![];
    for n in h1.iter() {
        if !h2.contains(n) {
            v1.push(*n);
        }
    }

    let mut v2 = vec![];
    for n in h2.iter() {
        if !h1.contains(n) {
            v2.push(*n);
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    vec![v1, v2]
}

fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let ret = find_differenct(nums1, nums2);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_differenct() {
    {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let expected = vec![vec![1, 3], vec![4, 6]];
        let ret = find_differenct(nums1, nums2);
        assert_eq!(ret, expected);
    }
    {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let expected = vec![vec![3], vec![]];
        let ret = find_differenct(nums1, nums2);
        assert_eq!(ret, expected);
    }
}
