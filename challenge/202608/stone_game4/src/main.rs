fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    let mut dp = vec![false; n + 1];

    for i in 0..=n {
        let mut j = 1;
        while j * j <= i {
            if !dp[i - (j * j)] {
                dp[i] = true;
                break;
            }

            j += 1;
        }
    }

    dp[n]
}

fn main() {
    let ret = winner_square_game(15);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(winner_square_game(1));
    assert!(!winner_square_game(2));
    assert!(winner_square_game(4));
}
