fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.windows(3).any(|v| v.iter().all(|n| n % 2 == 1))
}

fn main() {
    let arr = vec![2, 6, 4, 1];
    let ret = three_consecutive_odds(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![2, 6, 4, 1];
        assert!(!three_consecutive_odds(arr));
    }
    {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        assert!(three_consecutive_odds(arr));
    }
}
