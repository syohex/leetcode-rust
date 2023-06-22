fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut deposit = 0;
    let mut spent = -prices[0];

    for price in prices.into_iter().skip(1) {
        deposit = std::cmp::max(deposit, spent + price - fee);
        spent = std::cmp::max(spent, deposit - price);
    }

    deposit
}

fn main() {
    let prices = vec![1, 3, 2, 8, 4, 9];
    let ret = max_profit(prices, 2);
    println!("ret={ret}");
}

#[test]
fn test_max_profit() {
    {
        let prices = vec![1, 3, 2, 8, 4, 9];
        let ret = max_profit(prices, 2);
        assert_eq!(ret, 8);
    }
    {
        let prices = vec![1, 3, 7, 5, 10, 3];
        let ret = max_profit(prices, 3);
        assert_eq!(ret, 6);
    }
}
