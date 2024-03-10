fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let mut ret = vec![];
    let mut checked = HashSet::new();
    for n1 in &nums1 {
        if checked.contains(n1) {
            continue;
        }

        for n2 in &nums2 {
            if *n1 == *n2 {
                ret.push(*n1);
                break;
            }
        }

        checked.insert(*n1);
    }

    ret
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let ret = intersection(nums1, nums2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let expected = vec![2];
        let ret = intersection(nums1, nums2);
        assert_eq!(ret, expected);
    }
    {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![4, 9];
        let ret = intersection(nums1, nums2);
        assert_eq!(ret, expected);
    }
}
