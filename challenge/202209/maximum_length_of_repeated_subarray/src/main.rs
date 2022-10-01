fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    let mut ret = 0;
    for i in (0..len1).rev() {
        for j in (0..len2).rev() {
            if nums1[i] == nums2[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
                ret = std::cmp::max(ret, dp[i][j]);
            }
        }
    }

    ret
}

fn main() {
    let nums1 = vec![1, 2, 3, 2, 1];
    let nums2 = vec![3, 2, 1, 4, 7];
    let ret = find_length(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test_find_length() {
    {
        let nums1 = vec![1, 2, 3, 2, 1];
        let nums2 = vec![3, 2, 1, 4, 7];
        let ret = find_length(nums1, nums2);
        assert_eq!(ret, 3);
    }
    {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![0, 0, 0, 0, 0];
        let ret = find_length(nums1, nums2);
        assert_eq!(ret, 5);
    }
}
