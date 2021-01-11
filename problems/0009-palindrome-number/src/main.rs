fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

fn main() {
    println!("is_palindrome(121)={}", is_palindrome(121));
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(!is_palindrome(-101));
}
