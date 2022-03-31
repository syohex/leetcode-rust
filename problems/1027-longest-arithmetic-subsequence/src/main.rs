fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut dp = vec![vec![0; nums.len()]; 1001];
    let mut ret = std::i32::MIN;

    for i in 0..nums.len() {
        for j in 0..i {
            let diff = nums[i] - nums[j] + 500;
            dp[diff as usize][i] = std::cmp::max(2, dp[diff as usize][j] + 1);
            ret = std::cmp::max(ret, dp[diff as usize][i]);
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 6, 8, 12];
    let ret = longest_arith_seq_length(nums);
    println!("ret={ret}");
}

#[test]
fn test_longest_arith_seq_length() {
    {
        let nums = vec![3, 6, 9, 12];
        assert_eq!(longest_arith_seq_length(nums), 4);
    }
    {
        let nums = vec![9, 4, 7, 2, 10];
        assert_eq!(longest_arith_seq_length(nums), 3);
    }
    {
        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        assert_eq!(longest_arith_seq_length(nums), 4);
    }
}
