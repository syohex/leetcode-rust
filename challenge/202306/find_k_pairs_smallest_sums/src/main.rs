fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut q = BinaryHeap::new();
    q.push(Reverse((nums1[0] + nums2[0], 0, 0)));

    let mut checked = HashSet::new();
    checked.insert((0, 0));

    let k = k as usize;
    let mut ret = vec![];
    while let Some(Reverse((_, i, j))) = q.pop() {
        ret.push(vec![nums1[i], nums2[j]]);
        if ret.len() >= k {
            break;
        }

        if i + 1 < len1 && !checked.contains(&(i + 1, j)) {
            q.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
            checked.insert((i + 1, j));
        }

        if j + 1 < len2 && !checked.contains(&(i, j + 1)) {
            q.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            checked.insert((i, j + 1));
        }
    }

    ret
}

fn main() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let ret = k_smallest_pairs(nums1, nums2, 3);
    println!("ret={ret:?}");
}

#[test]
fn test_k_smallest_pairs() {
    {
        let nums1 = vec![1, 7, 11];
        let nums2 = vec![2, 4, 6];
        let expected = vec![vec![1, 2], vec![1, 4], vec![1, 6]];
        let ret = k_smallest_pairs(nums1, nums2, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let expected = vec![vec![1, 1], vec![1, 1]];
        let ret = k_smallest_pairs(nums1, nums2, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums1 = vec![1, 2];
        let nums2 = vec![3];
        let expected = vec![vec![1, 3], vec![2, 3]];
        let ret = k_smallest_pairs(nums1, nums2, 3);
        assert_eq!(ret, expected);
    }
}
