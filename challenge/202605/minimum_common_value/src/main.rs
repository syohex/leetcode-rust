fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < nums1.len() && i2 < nums2.len() {
        if nums1[i1] == nums2[i2] {
            return nums1[i1];
        }

        if nums1[i1] > nums2[i2] {
            i2 += 1;
        } else {
            i1 += 1;
        }
    }

    -1
}

fn main() {
    let ret = get_common(vec![1, 2, 3], vec![2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(get_common(vec![1, 2, 3], vec![2, 3]), 2);
    assert_eq!(get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    assert_eq!(get_common(vec![1, 2, 3], vec![4, 5, 6]), -1);
}
