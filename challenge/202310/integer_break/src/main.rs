fn integer_break(n: i32) -> i32 {
    use std::collections::HashMap;

    fn f(n: i32, orig: i32, acc: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return acc;
        }

        if let Some(&v) = cache.get(&n) {
            return v * acc;
        }

        let mut ret = 1;
        let limit = if n == orig { n } else { n + 1 };
        for i in 1..limit {
            ret = std::cmp::max(ret, f(n - i, orig, acc * i, cache));
        }

        cache.insert(n, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(n, n, 1, &mut cache)
}

fn main() {
    let ret = integer_break(58);
    println!("ret={ret}");
}

#[test]
fn test_integer_break() {
    {
        let ret = integer_break(2);
        assert_eq!(ret, 1);
    }
    {
        let ret = integer_break(4);
        assert_eq!(ret, 4);
    }
    {
        let ret = integer_break(10);
        assert_eq!(ret, 36);
    }
}
