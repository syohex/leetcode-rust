fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut prices = prices;
    prices.sort_unstable();

    let sum = prices[0] + prices[1];
    if sum > money {
        money
    } else {
        money - sum
    }
}

fn main() {
    let prices = vec![1, 2, 2];
    let ret = buy_choco(prices, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let prices = vec![1, 2, 2];
        let ret = buy_choco(prices, 3);
        assert_eq!(ret, 0);
    }
    {
        let prices = vec![3, 2, 3];
        let ret = buy_choco(prices, 3);
        assert_eq!(ret, 3);
    }
}
