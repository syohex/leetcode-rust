fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
    use std::collections::HashMap;

    const MOD: i32 = 1_000_000_007;
    let len = arr.len();
    let mut ret = 0;
    let mut h = HashMap::new();

    for i in 0..len - 1 {
        for j in i + 1..len {
            let diff = target - arr[i] - arr[j];
            if let Some(n) = h.get(&diff) {
                ret += n;
                ret %= MOD;
            }
        }

        *h.entry(arr[i]).or_insert(0) += 1;
    }

    ret
}

fn main() {
    let arr = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let ret = three_sum_multi(arr, 8);
    println!("ret={ret}");
}

#[test]
fn test_three_sum_multi() {
    {
        let arr = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(three_sum_multi(arr, 8), 20);
    }
    {
        let arr = vec![1, 1, 2, 2, 2, 2];
        assert_eq!(three_sum_multi(arr, 5), 12);
    }
}
