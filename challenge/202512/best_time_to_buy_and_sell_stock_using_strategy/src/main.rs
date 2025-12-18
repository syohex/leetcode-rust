fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let len = prices.len();
    let mut price_prefix = vec![0i64; len + 1];
    let mut profit_prefix = vec![0i64; len + 1];

    for (i, p) in prices.into_iter().enumerate() {
        price_prefix[i + 1] = price_prefix[i] + p as i64;
        profit_prefix[i + 1] = profit_prefix[i] + (p * strategy[i]) as i64;
    }

    let k = k as usize;
    let mut ret = profit_prefix[len];
    for i in (k - 1)..len {
        let left_value = profit_prefix[i + 1 - k];
        let right_value = profit_prefix[len] - profit_prefix[i + 1];
        let middle_value = price_prefix[i + 1] - price_prefix[i - (k / 2) + 1];
        ret = std::cmp::max(ret, left_value + middle_value + right_value);
    }

    ret
}

fn main() {
    let prices = vec![4, 2, 8];
    let strategy = vec![-1, 0, 1];
    let ret = max_profit(prices, strategy, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let prices = vec![4, 2, 8];
        let strategy = vec![-1, 0, 1];
        let ret = max_profit(prices, strategy, 2);
        assert_eq!(ret, 10);
    }
    {
        let prices = vec![5, 4, 3];
        let strategy = vec![1, 1, 0];
        let ret = max_profit(prices, strategy, 2);
        assert_eq!(ret, 9);
    }
}
