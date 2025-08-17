fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let n = n as usize;
    let k = k as usize;
    let max_pts = max_pts as usize;
    let mut dp = vec![0.0; n+1];
    dp[0] = 1.0;

    for i in 1..=n {
        for j in 1..=max_pts {
            if i >= j && i - j < k {
                dp[i] += dp[i - j] / max_pts as f64;
            }
        }
    }

    dp.into_iter().skip(k).sum()
}

fn main() {
    let ret = new21_game(21, 17, 10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(new21_game(10, 1, 10), 1.0);
    assert_eq!(new21_game(6, 1, 10), 0.6);
}
