fn is_subsequence(s: String, t: String) -> bool {
    let mut start = 0;
    for c in s.chars() {
        if let Some(pos) = t.chars().skip(start).position(|d| c == d) {
            start = start + pos + 1;
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let ret = is_subsequence("abc".to_string(), "ahbgdc".to_string());
    println!("ret={ret}");
}

#[test]
fn test_is_subsequence() {
    assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    assert!(!is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()));
    assert!(is_subsequence("ab".to_string(), "baab".to_string()));
}
