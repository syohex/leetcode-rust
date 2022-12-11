fn maximum_value(strs: Vec<String>) -> i32 {
    fn f(s: &str) -> Option<i32> {
        let mut ret = 0;
        for b in s.bytes() {
            if !(b >= b'0' && b <= b'9') {
                return None;
            }

            ret = ret * 10 + (b - b'0') as i32;
        }

        Some(ret)
    }

    strs.into_iter().fold(0, |acc, s| {
        if let Some(n) = f(&s) {
            std::cmp::max(acc, n)
        } else {
            std::cmp::max(acc, s.len() as i32)
        }
    })
}

fn main() {
    let strs = vec![
        "alic3".to_string(),
        "bob".to_string(),
        "3".to_string(),
        "4".to_string(),
        "00000".to_string(),
    ];
    let ret = maximum_value(strs);
    println!("ret={ret}");
}

#[test]
fn test_maximum_value() {
    {
        let strs = vec![
            "alic3".to_string(),
            "bob".to_string(),
            "3".to_string(),
            "4".to_string(),
            "00000".to_string(),
        ];
        let ret = maximum_value(strs);
        assert_eq!(ret, 5);
    }
    {
        let strs = vec![
            "1".to_string(),
            "01".to_string(),
            "001".to_string(),
            "0001".to_string(),
        ];
        let ret = maximum_value(strs);
        assert_eq!(ret, 1);
    }
}
