fn count_vowel_permutation(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0i64; n]; 5];
    const MOD: i64 = 1_000_000_007;

    // index: 0=a, 1=e, 2=i, 3=o, 4=u
    for i in 0..5 {
        dp[i][0] = 1;
    }

    for i in 1..n {
        dp[0][i] = dp[1][i - 1] % MOD;
        dp[1][i] = (dp[0][i - 1] + dp[2][i - 1]) % MOD;
        dp[2][i] = (dp[0][i - 1] + dp[1][i - 1] + dp[3][i - 1] + dp[4][i - 1]) % MOD;
        dp[3][i] = (dp[2][i - 1] + dp[4][i - 1]) % MOD;
        dp[4][i] = dp[0][i - 1] % MOD;
    }

    let mut ret = 0;
    for i in 0..5 {
        ret += dp[i][n - 1];
    }

    (ret % MOD) as i32
}

fn main() {
    let ret = count_vowel_permutation(5);
    println!("ret={ret}");
}

#[test]
fn test_count_vowel_permutation() {
    {
        let ret = count_vowel_permutation(1);
        assert_eq!(ret, 5);
    }
    {
        let ret = count_vowel_permutation(2);
        assert_eq!(ret, 10);
    }
    {
        let ret = count_vowel_permutation(5);
        assert_eq!(ret, 68);
    }
}
