fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let len = s.len();
    let a: Vec<_> = s.chars().collect();
    let b: Vec<_> = goal.chars().collect();

    for i in 0..len {
        let mut ok = true;
        for j in 0..len {
            if a[(j + i) % len] != b[j] {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}
fn main() {
    let ret = rotate_string("abcde".to_string(), "cdeab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(rotate_string("abcde".to_string(), "cdeab".to_string()));
    assert!(!rotate_string("abcde".to_string(), "abced".to_string()));
}
