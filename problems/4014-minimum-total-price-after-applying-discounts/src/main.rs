fn min_price(prices: Vec<i32>, discounts: Vec<i32>) -> f64 {
    let mut prices = prices;
    let mut discounts = discounts;

    prices.sort_unstable_by_key(|n| std::cmp::Reverse(*n));
    discounts.sort_unstable_by_key(|n| std::cmp::Reverse(*n));

    let len = std::cmp::max(prices.len(), discounts.len());
    let mut ret = 0.0;
    for i in 0..len {
        let price = *prices.get(i).unwrap_or(&0);
        let discount = *discounts.get(i).unwrap_or(&0);

        ret += (price * (100 - discount)) as f64 / 100.0;
    }

    ret
}

fn main() {
    let ret = min_price(vec![10, 30, 21], vec![50, 60]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_price(vec![10, 30, 21], vec![50, 60]), 32.5);
    assert_eq!(min_price(vec![100, 70], vec![10, 40, 50]), 92.0);
    assert_eq!(min_price(vec![7, 3, 9], vec![100, 100]), 3.0);
}
