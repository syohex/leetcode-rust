fn remove_palindrome_sub(s: String) -> i32 {
    let r: String = s.chars().rev().collect();
    if s == r {
        1
    } else {
        2
    }
}

fn main() {
    let ret = remove_palindrome_sub("ababa".to_string());
    println!("ret={ret}");
}

#[test]
fn test_remove_palidrome_sub() {
    assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
    assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
    assert_eq!(remove_palindrome_sub("baabb".to_string()), 2);
}
