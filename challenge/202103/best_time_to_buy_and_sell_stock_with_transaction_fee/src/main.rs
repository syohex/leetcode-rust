fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    use std::cmp::max;

    let mut money = 0;
    let mut used = -1 * prices[0];

    for &price in prices.iter().skip(1) {
        money = max(money, used + price - fee);
        used = max(used, money - price)
    }

    money
}

fn main() {
    let ret = max_profit(vec![1, 3, 2, 8, 4, 9], 2);
    println!("ret={}", ret);
}

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    assert_eq!(max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
}
