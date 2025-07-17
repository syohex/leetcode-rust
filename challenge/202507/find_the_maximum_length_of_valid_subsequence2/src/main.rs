fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut ret = 0;

    for i in 0..k {
        let mut dp = vec![0; k as usize];
        for &num in &nums {
            let m = num % k;
            let v = dp[((i + k - m) % k) as usize] + 1;
            dp[m as usize] = std::cmp::max(dp[m as usize], v);
            ret = std::cmp::max(ret, dp[m as usize]);
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 4, 2, 3, 1, 4];
    let ret = maximum_length(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = maximum_length(nums, 2);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 4, 2, 3, 1, 4];
        let ret = maximum_length(nums, 3);
        assert_eq!(ret, 4);
    }
}
