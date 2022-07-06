fn fib(n: i32) -> i32 {
    use std::collections::HashMap;

    fn f(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                if let Some(&v) = cache.get(&n) {
                    v
                } else {
                    let ret = f(n - 1, cache) + f(n - 2, cache);
                    cache.insert(n, ret);
                    ret
                }
            }
        }
    }

    let mut cache = HashMap::new();
    f(n, &mut cache)
}

fn main() {
    let ret = fib(20);
    println!("fib(20) = {ret}");
}

#[test]
fn test_fib() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(20), 6765);
}
