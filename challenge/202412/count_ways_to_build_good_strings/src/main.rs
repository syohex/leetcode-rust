fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let modulo = 1_000_000_007i64;
    let (low, high, zero, one) = (low as usize, high as usize, zero as usize, one as usize);
    let mut dp = vec![0i64; high + 1];
    dp[0] = 1;

    for i in 1..=high {
        if i >= zero {
            dp[i] = dp[i - zero];
        }
        if i >= one {
            dp[i] = (dp[i] + dp[i - one]) % modulo;
        }
    }

    let mut ret = 0i64;
    for i in low..=high {
        ret = (ret + dp[i]) % modulo;
    }

    ret as i32
}

fn main() {
    let ret = count_good_strings(3, 3, 1, 1);
    println!("ret={ret}");

    let ret = count_good_strings(5000, 10000, 5, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_good_strings(3, 3, 1, 1), 8);
    assert_eq!(count_good_strings(2, 3, 1, 2), 5);
}
