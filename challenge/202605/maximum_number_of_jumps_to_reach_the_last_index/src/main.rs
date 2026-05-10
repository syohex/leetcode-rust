fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut dp = vec![i32::MIN; len];
    dp[0] = 0;

    for i in 1..len {
        for j in 0..i {
            if (nums[i] - nums[j]).abs() <= target {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }

    if dp[len - 1] < 0 {
        -1
    } else {
        dp[len - 1]
    }
}

fn main() {
    let ret = maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_jumps(vec![0, 2, 1, 3], 1), -1);
    assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
    assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
    assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
}
