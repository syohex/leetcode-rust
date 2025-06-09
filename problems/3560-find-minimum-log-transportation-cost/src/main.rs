fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
    fn f(n: i64, k: i64) -> i64 {
        if n <= k {
            0
        } else {
            let limit = n / 2;
            let mut ret = i64::MAX;
            for i in 1..=limit {
                if i <= k && (n - i) <= k {
                    ret = std::cmp::min(ret, i * (n - i));
                }
            }
            ret
        }
    }

    let n = n as i64;
    let m = m as i64;
    let k = k as i64;

    f(n, k) + f(m, k)
}

fn main() {
    let ret = min_cutting_cost(6, 5, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_cutting_cost(6, 5, 5), 5);
    assert_eq!(min_cutting_cost(4, 4, 6), 0);
}
