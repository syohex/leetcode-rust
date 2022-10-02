fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    use std::collections::HashMap;

    fn f(
        i: i32,
        sum: i32,
        n: i32,
        k: i32,
        target: i32,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if i == n {
            if sum == target {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(v) = cache.get(&(i, sum)) {
            return *v;
        }

        let mut ret = 0;
        let modulo = 1_000_000_007;
        let len = std::cmp::min(k, target - sum);
        for j in 1..=len {
            ret = (ret + f(i + 1, sum + j, n, k, target, cache)) % modulo;
        }

        cache.insert((i, sum), ret);
        ret
    }

    let mut cache = HashMap::new();
    f(0, 0, n, k, target, &mut cache)
}

fn main() {
    let ret = num_rolls_to_target(30, 30, 500);
    println!("ret={ret}");
}

#[test]
fn test_num_rolls_to_target() {
    {
        let ret = num_rolls_to_target(1, 6, 3);
        assert_eq!(ret, 1);
    }
    {
        let ret = num_rolls_to_target(2, 6, 7);
        assert_eq!(ret, 6);
    }
    {
        let ret = num_rolls_to_target(30, 30, 500);
        assert_eq!(ret, 222616187);
    }
}
