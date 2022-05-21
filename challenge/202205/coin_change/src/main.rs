fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in &coins {
            if i >= coin {
                dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coin) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == amount + 1 {
        -1
    } else {
        dp[amount as usize]
    }
}

fn main() {
    let coins = vec![1, 2, 5];
    let ret = coin_change(coins, 11);
    println!("ret={ret}");
}

#[test]
fn test_coin_change() {
    {
        let coins = vec![1, 2, 5];
        assert_eq!(coin_change(coins, 11), 3);
    }
    {
        let coins = vec![2];
        assert_eq!(coin_change(coins, 3), -1);
    }
    {
        let coins = vec![1];
        assert_eq!(coin_change(coins, 0), 0);
    }
    {
        let coins = vec![186, 419, 83, 408];
        assert_eq!(coin_change(coins, 6249), 20);
    }
}
