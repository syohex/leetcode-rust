fn clear_digits(s: String) -> String {
    let mut ret = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            ret.pop();
        } else {
            ret.push(c);
        }
    }
    ret
}

fn main() {
    let ret = clear_digits("abcd9e".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(clear_digits("abc".to_string()), "abc");
    assert_eq!(clear_digits("cb34".to_string()), "");
}
