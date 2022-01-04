fn check_string(s: String) -> bool {
    let last_a = s.bytes().rposition(|c| c == b'a');
    let first_b = s.bytes().position(|c| c == b'b');

    match (last_a, first_b) {
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => true,
        (Some(a), Some(b)) => a < b,
    }
}

fn main() {
    println!("ret={}", check_string("aaabbb".to_string()));
}

#[test]
fn test_check_string() {
    assert!(check_string("aaabbb".to_string()));
    assert!(!check_string("abab".to_string()));
    assert!(check_string("bbb".to_string()));
}
