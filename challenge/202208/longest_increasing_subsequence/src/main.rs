fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    dp[0] = 1;

    let mut ret = 1;
    for i in 1..nums.len() {
        dp[i] = 1;
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }

        ret = std::cmp::max(ret, dp[i]);
    }

    ret
}

fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let ret = length_of_lis(nums);
    println!("ret={ret}");
}

#[test]
fn test_length_of_lis() {
    {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let ret = length_of_lis(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let ret = length_of_lis(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        let ret = length_of_lis(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
        let ret = length_of_lis(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![0];
        let ret = length_of_lis(nums);
        assert_eq!(ret, 1);
    }
}
