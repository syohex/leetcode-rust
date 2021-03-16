fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::max;

    let mut ret = 0;
    let mut buy_price = prices[0];

    for &price in prices.iter().skip(1) {
        if price > buy_price {
            ret = max(ret, price - buy_price);
        } else {
            buy_price = price
        }
    }

    ret
}

fn main() {
    let ret = max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("ret={}", ret);
}

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
