fn clear_digits(s: String) -> String {
    let mut ret = String::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            ret.push(c);
        } else {
            ret.pop();
        }
    }
    ret
}
fn main() {
    let s = "cb34".to_string();
    let ret = clear_digits(s);
    println!("ret='{ret}'");
}

#[test]
fn test() {
    {
        let s = "abc".to_string();
        let ret = clear_digits(s);
        assert_eq!(ret, "abc");
    }
    {
        let s = "cb34".to_string();
        let ret = clear_digits(s);
        assert_eq!(ret, "");
    }
    {
        let s = "1234".to_string();
        let ret = clear_digits(s);
        assert_eq!(ret, "");
    }
}
