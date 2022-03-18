fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let mut ret = 0;
    let mut has_odd = false;
    for (_, &v) in &h {
        if v % 2 == 0 {
            ret += v;
        } else {
            ret += v - 1;
            if !has_odd {
                ret += 1;
                has_odd = true;
            }
        }
    }

    ret
}

fn main() {
    let ret = longest_palindrome("abccccdd".to_string());
    println!("ret={ret}");
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(longest_palindrome("a".to_string()), 1);
    assert_eq!(longest_palindrome("bb".to_string()), 2);
}
