fn valid_palindrome(s: String) -> bool {
    fn is_palidrome(s: &[char], left: i32, right: i32) -> bool {
        let mut left = left;
        let mut right = right;

        while left < right {
            if s[left as usize] != s[right as usize] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    let cs = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = cs.len() as i32 - 1;

    while left < right {
        if cs[left as usize] != cs[right as usize] {
            return is_palidrome(&cs, left + 1, right) || is_palidrome(&cs, left, right - 1);
        }

        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    let ret = valid_palindrome("deeee".to_string());
    println!("ret={ret}");
}

#[test]
fn test_valid_palindrome() {
    assert!(valid_palindrome("aba".to_string()));
    assert!(valid_palindrome("abca".to_string()));
    assert!(!valid_palindrome("abc".to_string()));
    assert!(valid_palindrome("deeee".to_string()));
}
