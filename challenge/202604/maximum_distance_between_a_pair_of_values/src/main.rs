fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums2: Vec<_> = nums2.into_iter().enumerate().rev().collect();

    let mut ret = 0;
    for (i, n) in nums1.into_iter().enumerate() {
        let pos = nums2.partition_point(|(_, m)| *m < n);
        if pos < nums2.len() && nums2[pos].0 >= i {
            ret = std::cmp::max(ret, (nums2[pos].0 - i) as i32);
        }
    }

    ret
}

fn main() {
    let ret = max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
        2
    );
    assert_eq!(max_distance(vec![2, 2, 2], vec![10, 10, 1]), 1);
    assert_eq!(
        max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]),
        2
    );
    assert_eq!(max_distance(vec![1, 1, 1], vec![0, 0, 0]), 0);
}
