fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let mut ret = vec![];
    let mut tmp = String::new();
    let k = k as usize;
    let len = s.len();

    for c in s.chars() {
        if tmp.len() == k {
            ret.push(tmp);
            tmp = String::new();
        }

        tmp.push(c);
    }

    if len % k != 0 {
        let remaining = k - len % k;
        for _ in 0..remaining {
            tmp.push(fill);
        }
        ret.push(tmp);
    } else {
        ret.push(tmp);
    }

    ret
}

fn main() {
    let s = "abcdefghi".to_string();
    let ret = divide_string(s, 3, 'x');
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let s = "abcdefghi".to_string();
        let expected = vec!["abc", "def", "ghi"];
        let ret = divide_string(s, 3, 'x');
        assert_eq!(ret, expected);
    }

    {
        let s = "abcdefghij".to_string();
        let expected = vec!["abc", "def", "ghi", "jxx"];
        let ret = divide_string(s, 3, 'x');
        assert_eq!(ret, expected);
    }
}
