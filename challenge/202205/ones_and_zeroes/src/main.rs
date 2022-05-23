fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    use std::collections::HashMap;

    fn f(
        i: usize,
        bits: &Vec<Vec<i32>>,
        zeros: i32,
        ones: i32,
        cache: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if i >= bits.len() {
            return 0;
        }

        if let Some(v) = cache.get(&(i, zeros, ones)) {
            *v
        } else {
            let ret1 = if zeros - bits[i][0] >= 0 && ones - bits[i][1] >= 0 {
                f(i + 1, bits, zeros - bits[i][0], ones - bits[i][1], cache) + 1
            } else {
                0
            };
            let ret2 = f(i + 1, bits, zeros, ones, cache);

            let ret = std::cmp::max(ret1, ret2);
            cache.insert((i, zeros, ones), ret);
            ret
        }
    }

    let mut bits = Vec::new();
    for s in &strs {
        let mut v = vec![0; 2];
        for c in s.chars() {
            if c == '0' {
                v[0] += 1;
            } else {
                v[1] += 1;
            }
        }

        bits.push(v);
    }

    let mut cache = HashMap::new();
    f(0, &bits, m, n, &mut cache)
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
fn test_find_max_form() {
    {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        assert_eq!(find_max_form(strs, 5, 3), 4);
    }
    {
        let strs = vec!["10".to_string(), "1".to_string(), "0".to_string()];
        assert_eq!(find_max_form(strs, 1, 1), 2);
    }
    {
        let strs = vec![
            "011".to_string(),
            "1".to_string(),
            "11".to_string(),
            "0".to_string(),
            "010".to_string(),
            "1".to_string(),
            "10".to_string(),
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
            "01111".to_string(),
            "011".to_string(),
            "11".to_string(),
            "00".to_string(),
            "11".to_string(),
            "10".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
            "101".to_string(),
            "001110".to_string(),
            "1".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "10".to_string(),
            "00100".to_string(),
            "0".to_string(),
            "10".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "011".to_string(),
            "11".to_string(),
            "11".to_string(),
            "10".to_string(),
            "10".to_string(),
            "0000".to_string(),
            "01".to_string(),
            "1".to_string(),
            "10".to_string(),
            "0".to_string(),
        ];
        assert_eq!(find_max_form(strs, 44, 39), 45);
    }
}
