fn is_palindrome(s: String) -> bool {
    let s = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();
    let r = s.chars().rev().collect::<String>();

    s == r
}

fn main() {
    println!(
        "is_palindrome('race a car')={}",
        is_palindrome("race a car".to_string())
    );
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!is_palindrome("race a car".to_string()));
    assert!(is_palindrome("".to_string()));
    assert!(!is_palindrome("1A".to_string()));
}
