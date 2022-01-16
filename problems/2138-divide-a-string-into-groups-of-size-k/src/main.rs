fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let mut ret = vec![];
    let cs: Vec<char> = s.chars().collect();
    let k = k as usize;

    let count = s.len() / k;
    let mut pos = 0;
    for _ in 0..count {
        let mut tmp = String::new();
        for j in pos..pos + k {
            tmp.push(cs[j]);
        }

        ret.push(tmp);
        pos += k;
    }

    let rem = s.len() % k;
    if rem != 0 {
        let mut tmp = String::new();
        for i in pos..pos + rem {
            tmp.push(cs[i]);
        }
        for _ in 0..(k - rem) {
            tmp.push(fill);
        }
        ret.push(tmp);
    }

    ret
}

fn main() {
    let s = "abcdefghi".to_string();
    let ret = divide_string(s, 3, 'x');
    println!("{:?}", ret);
}

#[test]
fn test_divide_string() {
    {
        let s = "abcdefghi".to_string();
        let expected = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let ret = divide_string(s, 3, 'x');
        assert_eq!(ret, expected);
    }
    {
        let s = "abcdefghij".to_string();
        let expected = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "jxx".to_string(),
        ];
        let ret = divide_string(s, 3, 'x');
        assert_eq!(ret, expected);
    }
}
