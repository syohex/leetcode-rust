fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut nums1 = nums1;
    let mut nums2 = nums2;

    nums1.sort_unstable();
    nums2.sort_unstable();

    for num1 in &nums1 {
        for num2 in &nums2 {
            if *num1 == *num2 {
                return *num1;
            }
        }
    }

    if nums1[0] < nums2[0] {
        nums1[0] * 10 + nums2[0]
    } else {
        nums2[0] * 10 + nums1[0]
    }
}

fn main() {
    let nums1 = vec![4, 1, 3];
    let nums2 = vec![4, 1, 3];
    let ret = min_number(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test_min_number() {
    {
        let nums1 = vec![4, 1, 3];
        let nums2 = vec![5, 7];
        let ret = min_number(nums1, nums2);
        assert_eq!(ret, 15);
    }
    {
        let nums1 = vec![3, 5, 2, 6];
        let nums2 = vec![3, 1, 7];
        let ret = min_number(nums1, nums2);
        assert_eq!(ret, 3);
    }
}
