fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut arr = arr;
    arr.sort_unstable();

    let mut h = HashMap::new();
    for (i, arr) in arr.iter().enumerate() {
        h.insert(*arr, i);
    }

    let mut dp = vec![1i64; arr.len()];
    const MODULO: i64 = 1_000_000_007;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                if let Some(k) = h.get(&(arr[i] / arr[j])) {
                    dp[i] = (dp[i] + (dp[j] * dp[*k])) % MODULO;
                }
            }
        }
    }

    dp.into_iter().fold(0i64, |acc, num| (acc + num) % MODULO) as i32
}

fn main() {
    let arr = vec![2, 4, 5, 10];
    let ret = num_factored_binary_trees(arr);
    println!("ret={ret}");
}

#[test]
fn test_num_factored_binary_trees() {
    {
        let arr = vec![2, 4];
        let ret = num_factored_binary_trees(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![2, 4, 5, 10];
        let ret = num_factored_binary_trees(arr);
        assert_eq!(ret, 7);
    }
}
