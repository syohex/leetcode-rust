fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
    dp[0][0] = 1;
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }

            if i >= 1 {
                dp[i][j] += dp[i - 1][j];
            }
            if j >= 1 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    dp[m - 1][n - 1]
}

fn main() {
    println!("unique_paths(3, 7)={}", unique_paths(3, 7));
}

#[test]
fn test_unique_paths() {
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
    assert_eq!(unique_paths(3, 3), 6);
}
