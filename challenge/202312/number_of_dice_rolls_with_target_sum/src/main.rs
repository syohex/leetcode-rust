fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    use std::collections::HashMap;

    fn f(n: i32, k: i32, sum: i32, target: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if n == 0 {
            if sum == target {
                return 1;
            }

            return 0;
        }
        
        if sum > target {
            return 0;
        }

        if let Some(&v) = cache.get(&(n, sum)) {
            return v;
        }

        let mut ret = 0;
        let limit = std::cmp::min(k, target - sum);
        for i in 1..=limit {
            ret = (ret + f(n - 1, k, sum + i, target, cache)) % 1_000_000_007;
        }

        cache.insert((n, sum), ret);
        ret
    }

    let mut cache = HashMap::new();
    f(n, k, 0, target, &mut cache)
}

fn main() {
    let ret = num_rolls_to_target(30, 30, 500);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_rolls_to_target(1, 6, 3), 1);
    assert_eq!(num_rolls_to_target(2, 6, 7), 6);
    assert_eq!(num_rolls_to_target(30, 30, 500), 222616187);
}
