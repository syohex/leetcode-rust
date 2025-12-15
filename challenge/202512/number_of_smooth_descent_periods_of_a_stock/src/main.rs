fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut left = 0;
    let mut ret = 0i64;

    for i in 1..prices.len() {
        if prices[i] + 1 != prices[i - 1] {
            let count = (i - left) as i64;
            ret += (1 + count) * count / 2;
            left = i;
        }
    }

    let count = (prices.len() - left) as i64;
    ret += (1 + count) * count / 2;

    ret
}

fn main() {
    let ret = get_descent_periods(vec![3, 2, 1, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(get_descent_periods(vec![4, 3, 2, 1]), 10);
    assert_eq!(get_descent_periods(vec![3, 2, 1, 4]), 7);
    assert_eq!(get_descent_periods(vec![8, 6, 7, 7]), 4);
    assert_eq!(get_descent_periods(vec![1]), 1);
}
