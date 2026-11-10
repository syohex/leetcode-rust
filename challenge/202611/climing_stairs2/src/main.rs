fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        if i >= 1 {
            dp[i] = std::cmp::min(dp[i], dp[i - 1] + costs[i - 1] + 1);
        }
        if i >= 2 {
            dp[i] = std::cmp::min(dp[i], dp[i - 2] + costs[i - 1] + 4);
        }
        if i >= 3 {
            dp[i] = std::cmp::min(dp[i], dp[i - 3] + costs[i - 1] + 9);
        }
    }

    dp[n]
}

fn main() {
    let ret = climb_stairs(4, vec![1, 2, 3, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(climb_stairs(4, vec![1, 2, 3, 4]), 13);
    assert_eq!(climb_stairs(4, vec![5, 1, 6, 2]), 11);
    assert_eq!(climb_stairs(3, vec![9, 8, 3]), 12);
}
