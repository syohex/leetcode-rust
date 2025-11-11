fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        one_zero: &[(i32, i32)],
        m: i32,
        n: i32,
        cache: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if pos >= one_zero.len() {
            0
        } else {
            let key = (pos, m, n);
            if let Some(&v) = cache.get(&key) {
                v
            } else {
                let ret1 = if m - one_zero[pos].0 >= 0 && n - one_zero[pos].1 >= 0 {
                    f(
                        pos + 1,
                        one_zero,
                        m - one_zero[pos].0,
                        n - one_zero[pos].1,
                        cache,
                    ) + 1
                } else {
                    0
                };

                let ret2 = f(pos + 1, one_zero, m, n, cache);
                let ret = std::cmp::max(ret1, ret2);
                cache.insert(key, ret);
                ret
            }
        }
    }

    let one_zero: Vec<_> = strs
        .into_iter()
        .map(|s| {
            s.chars().fold((0, 0), |(zeros, ones), c| {
                if c == '0' {
                    (zeros + 1, ones)
                } else {
                    (zeros, ones + 1)
                }
            })
        })
        .collect();

    let mut cache = HashMap::new();
    f(0, &one_zero, m, n, &mut cache)
}

fn main() {
    let strs = vec![
        "10".to_string(),
        "0001".to_string(),
        "111001".to_string(),
        "1".to_string(),
        "0".to_string(),
    ];
    let ret = find_max_form(strs, 5, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let strs = vec!["10".to_string(), "0".to_string(), "1".to_string()];
        let ret = find_max_form(strs, 1, 1);
        assert_eq!(ret, 2);
    }
    {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let ret = find_max_form(strs, 5, 3);
        assert_eq!(ret, 4);
    }
}
