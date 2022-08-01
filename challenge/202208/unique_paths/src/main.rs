fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = 1;

    for i in 0..m {
        for j in 0..n {
            if i != 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j != 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    dp[m - 1][n - 1]
}

fn main() {
    let m = 3;
    let n = 7;
    let ret = unique_paths(m, n);
    println!("ret={ret}");
}

#[test]
fn test_unique_paths() {
    {
        let m = 3;
        let n = 7;
        let ret = unique_paths(m, n);
        assert_eq!(ret, 28);
    }
    {
        let m = 3;
        let n = 2;
        let ret = unique_paths(m, n);
        assert_eq!(ret, 3);
    }
    {
        let m = 1;
        let n = 10;
        let ret = unique_paths(m, n);
        assert_eq!(ret, 1);
    }
}
