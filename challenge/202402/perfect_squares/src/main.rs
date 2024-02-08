fn num_squares(n: i32) -> i32 {
    let mut squares = vec![];
    let mut i = 1;
    while i * i <= n {
        squares.push(i * i);
        i += 1;
    }

    let mut dp = vec![n + 1; n as usize + 1];
    dp[0] = 0;

    let n = n as usize;
    for i in 1..=n {
        for &square in &squares {
            let s = square as usize;
            if s > i {
                break;
            }

            dp[i] = std::cmp::min(dp[i], dp[i - s] + 1);
        }
    }

    dp[n]
}

fn main() {
    let ret = num_squares(12);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
