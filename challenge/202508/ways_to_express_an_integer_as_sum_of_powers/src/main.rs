fn number_of_ways(n: i32, x: i32) -> i32 {
    let modulo = 1_000_000_007;
    let n = n as usize;

    let mut powers = vec![];
    for i in 1..=n {
        let v = i.pow(x as u32);
        if v > n {
            break;
        }

        powers.push(v);
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for p in powers {
        for j in (p..=n).rev() {
            dp[j] = (dp[j] + dp[j - p]) % modulo;
        }
    }

    dp[n]
}

fn main() {
    let ret = number_of_ways(4, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(number_of_ways(10, 2), 1);
    assert_eq!(number_of_ways(4, 1), 2);
}
