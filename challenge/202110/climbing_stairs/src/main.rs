fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

fn main() {
    println!("climb_stairs(2)={}", climb_stairs(2));
}

#[test]
fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(38), 63245986);
}
