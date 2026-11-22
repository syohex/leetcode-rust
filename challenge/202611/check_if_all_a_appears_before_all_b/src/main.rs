fn check_string(s: String) -> bool {
    let mut has_b = false;
    for c in s.chars() {
        if c == 'b' {
            has_b = true;
        } else if has_b {
            return false;
        }
    }

    true
}

fn main() {
    let ret = check_string("aaabbb".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_string("aaabbb".to_string()));
    assert!(!check_string("abab".to_string()));
    assert!(check_string("bbb".to_string()));
}
