fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{max, min};

    let mut min_price = std::i32::MAX;
    let mut ret = 0;

    for price in &prices {
        min_price = min(min_price, *price);
        ret = max(ret, price - min_price);
    }

    ret
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let ret = max_profit(prices);
    println!("ret={ret}");
}

#[test]
fn test_max_profit() {
    {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5)
    }
    {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0)
    }
}
