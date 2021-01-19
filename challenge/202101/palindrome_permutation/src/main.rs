use std::collections::HashMap;

fn can_permute_palindrome(s: String) -> bool {
    let mut h: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let odds = h.iter().filter(|(_, &v)| v % 2 == 1).count();
    if s.len() % 2 == 0 {
        odds == 0
    } else {
        odds == 1
    }
}

fn main() {
    println!(
        "can_permute_palindrome('code')={}",
        can_permute_palindrome("code".to_string())
    );
}

#[test]
fn test_can_permute_palindrome() {
    assert!(!can_permute_palindrome("code".to_string()));
    assert!(can_permute_palindrome("aab".to_string()));
    assert!(can_permute_palindrome("carerac".to_string()));
}
