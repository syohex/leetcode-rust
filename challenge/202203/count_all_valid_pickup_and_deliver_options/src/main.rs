use std::collections::HashMap;

fn count_orders1(unpicked: i64, undelivered: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    const MOD: i64 = 1_000_000_007;

    if unpicked == 0 && undelivered == 0 {
        return 1;
    }

    if unpicked < 0 || undelivered < 0 || unpicked > undelivered {
        return 0;
    }

    if let Some(v) = cache.get(&(unpicked, undelivered)) {
        *v
    } else {
        let mut ret = unpicked * count_orders1(unpicked - 1, undelivered, cache);
        ret %= MOD;

        let picked = undelivered - unpicked;
        ret += picked * count_orders1(unpicked, undelivered - 1, cache);
        ret %= MOD;

        cache.insert((unpicked, undelivered), ret);

        ret
    }
}

fn count_orders(n: i32) -> i32 {
    let mut cache = HashMap::new();
    let n = n as i64;
    count_orders1(n, n, &mut cache) as i32
}

fn main() {
    println!("count_orders(3)={}", count_orders(3));
}

#[test]
fn test_count_orders() {
    assert_eq!(count_orders(1), 1);
    assert_eq!(count_orders(2), 6);
    assert_eq!(count_orders(3), 90);
    assert_eq!(count_orders(8), 729647433);
}
