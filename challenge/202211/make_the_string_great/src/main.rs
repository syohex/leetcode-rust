fn make_good(s: String) -> String {
    let mut ret = String::new();
    for c in s.chars() {
        if let Some(prev) = ret.chars().last() {
            if (c.is_ascii_uppercase() && prev == c.to_ascii_lowercase())
                || (c.is_ascii_lowercase() && prev == c.to_ascii_uppercase())
            {
                ret.pop();
                continue;
            }
        }

        ret.push(c);
    }

    ret
}

fn main() {
    let s = "leEeetcode".to_string();
    let ret = make_good(s);
    println!("ret={ret}");
}

#[test]
fn test_make_good() {
    {
        let s = "leEeetcode".to_string();
        let ret = make_good(s);
        assert_eq!(ret, "leetcode");
    }
    {
        let s = "abBAcC".to_string();
        let ret = make_good(s);
        assert_eq!(ret, "");
    }
    {
        let s = "Pp".to_string();
        let ret = make_good(s);
        assert_eq!(ret, "");
    }
    {
        let s = "pP".to_string();
        let ret = make_good(s);
        assert_eq!(ret, "");
    }
}
