use std::cmp::min;

fn num_squares(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![std::i32::MAX; (n + 1) as usize];
    dp[0] = 0;

    let limit = ((n as f64).sqrt() as i32) + 1;
    let squares: Vec<i32> = (1..limit).into_iter().map(|n| n * n).collect();

    for i in (1..=n).into_iter() {
        for square in squares.iter() {
            if square > &i {
                break;
            }

            dp[i as usize] = min(dp[i as usize], dp[(i - square) as usize] + 1);
        }
    }

    dp[n as usize]
}

fn main() {
    println!("num_squares(12) = {}", num_squares(12));
}

#[test]
fn test_num_squares() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
    assert_eq!(num_squares(2), 2);
}
