fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut arr = arr.clone();
    arr.sort_unstable();

    let mut h: HashMap<i32, usize> = HashMap::new();
    for i in 0..arr.len() {
        h.insert(arr[i], i);
    }

    let mut dp: Vec<i64> = vec![1; arr.len()];
    let modulo = 10_i64.pow(9) + 7;

    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                let other = arr[i] / arr[j];
                if h.contains_key(&other) {
                    let other_index = h.get(&other).unwrap();
                    dp[i] = (dp[i] + (dp[j] * dp[*other_index])) % modulo;
                }
            }
        }
    }

    let mut ret = 0_i64;
    for d in dp {
        ret += d;
    }

    (ret % modulo as i64) as i32
}

fn main() {
    let ret = num_factored_binary_trees(vec![2, 4]);
    println!("ret={}", ret);
}

#[test]
fn test_num_factored_binary_trees() {
    assert_eq!(num_factored_binary_trees(vec![2, 4]), 3);
    assert_eq!(num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
