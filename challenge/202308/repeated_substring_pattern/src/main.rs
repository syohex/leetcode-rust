fn repeated_substring_pattern(s: String) -> bool {
    let len = s.len();

    for i in 1..=(len / 2) {
        if len % i == 0 {
            let t = &s[0..i].repeat(len / i);
            if s == *t {
                return true;
            }
        }
    }

    false
}

fn main() {
    println!("ret={}", repeated_substring_pattern("abab".to_string()));
}

#[test]
fn test_repeated_substring_pattern() {
    {
        let s = "abab".to_string();
        assert!(repeated_substring_pattern(s));
    }
    {
        let s = "aba".to_string();
        assert!(!repeated_substring_pattern(s));
    }
    {
        let s = "abcabcabcabc".to_string();
        assert!(repeated_substring_pattern(s));
    }
}
