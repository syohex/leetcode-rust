fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = multipliers.len();
    let mut dp = vec![vec![0; m + 1]; m + 1];

    for i in (0..m).rev() {
        for left in (0..=i).rev() {
            let val1 = multipliers[i] * nums[left] + dp[i + 1][left + 1];
            let val2 = multipliers[i] * nums[n - 1 - (i - left)] + dp[i + 1][left];
            dp[i][left] = std::cmp::max(val1, val2);
        }
    }

    dp[0][0]
}

fn main() {
    let nums = vec![-5, -3, -3, -2, 7, 1];
    let multipliers = vec![-10, -5, 3, 4, 6];
    let ret = maximum_score(nums, multipliers);
    println!("ret={ret}");
}

#[test]
fn test_maxmium_score() {
    {
        let nums = vec![1, 2, 3];
        let multipliers = vec![3, 2, 1];
        let ret = maximum_score(nums, multipliers);
        assert_eq!(ret, 14);
    }
    {
        let nums = vec![-5, -3, -3, -2, 7, 1];
        let multipliers = vec![-10, -5, 3, 4, 6];
        let ret = maximum_score(nums, multipliers);
        assert_eq!(ret, 102);
    }
}
