fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for i in 0..prices.len() {
        match prices.iter().skip(i + 1).find(|&n| prices[i] >= *n) {
            Some(v) => ret.push(prices[i] - *v),
            None => ret.push(prices[i]),
        }
    }
    ret
}

fn main() {
    let prices = vec![8, 4, 6, 2, 3];
    let ret = final_prices(prices);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let prices = vec![8, 4, 6, 2, 3];
        let expected = vec![4, 2, 4, 2, 3];
        let ret = final_prices(prices);
        assert_eq!(ret, expected);
    }
    {
        let prices = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let ret = final_prices(prices);
        assert_eq!(ret, expected);
    }
    {
        let prices = vec![10, 1, 1, 6];
        let expected = vec![9, 0, 1, 6];
        let ret = final_prices(prices);
        assert_eq!(ret, expected);
    }
}
