fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|money| money.iter().sum())
        .max()
        .unwrap()
}

fn main() {
    let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
    let ret = maximum_wealth(accounts);
    println!("ret={ret}");
}

#[test]
fn test_maximum_wealth() {
    {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts), 6);
    }
    {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(maximum_wealth(accounts), 10);
    }
    {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(maximum_wealth(accounts), 17);
    }
}
