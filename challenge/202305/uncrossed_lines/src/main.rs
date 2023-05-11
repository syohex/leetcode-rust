fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 1..=len1 {
        for j in 1..=len2 {
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[len1][len2]
}
fn main() {
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    let ret = max_uncrossed_lines(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test_max_unscrossed_lines() {
    {
        let nums1 = vec![1, 4, 2];
        let nums2 = vec![1, 2, 4];
        let ret = max_uncrossed_lines(nums1, nums2);
        assert_eq!(ret, 2);
    }
    {
        let nums1 = vec![2, 5, 1, 2, 5];
        let nums2 = vec![10, 5, 2, 1, 5, 2];
        let ret = max_uncrossed_lines(nums1, nums2);
        assert_eq!(ret, 3);
    }
    {
        let nums1 = vec![1, 3, 7, 1, 7, 5];
        let nums2 = vec![1, 9, 2, 5, 1];
        let ret = max_uncrossed_lines(nums1, nums2);
        assert_eq!(ret, 2);
    }
}
