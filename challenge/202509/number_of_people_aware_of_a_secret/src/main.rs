fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let modulo = 1_000_000_007;
    let n = n as usize;
    let forget = forget as usize;
    let delay = delay as usize;

    let mut dp = vec![0; n];
    dp[0] = 1;

    for i in 0..n {
        let limit = std::cmp::min(i + forget, n);
        for j in (i + delay)..limit {
            dp[j] = (dp[j] + dp[i]) % modulo;
        }
    }

    let mut ret = 0;
    for v in dp.into_iter().skip(n - forget) {
        ret = (ret + v) % modulo;
    }

    ret
}

fn main() {
    let ret = people_aware_of_secret(6, 2, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(people_aware_of_secret(6, 2, 4), 5);
    assert_eq!(people_aware_of_secret(4, 1, 3), 6);
}
