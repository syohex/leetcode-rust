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
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4];
    let ret = get_common(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];
        let ret = get_common(nums1, nums2);
        assert_eq!(ret, 2);
    }
    {
        let nums1 = vec![1, 2, 3, 6];
        let nums2 = vec![2, 3, 4, 5];
        let ret = get_common(nums1, nums2);
        assert_eq!(ret, 2);
    }
    {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4];
        let ret = get_common(nums1, nums2);
        assert_eq!(ret, -1);
    }
}
