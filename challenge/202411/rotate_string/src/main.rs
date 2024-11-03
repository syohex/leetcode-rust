fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let len = s.len();
    let cs1: Vec<char> = s.chars().collect();
    let cs2: Vec<char> = goal.chars().collect();

    (0..len)
        .into_iter()
        .any(|i| (0..len).into_iter().all(|j| cs1[j] == cs2[(j + i) % len]))
}

fn main() {
    let ret = rotate_string("abcde".to_string(), "abced".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(rotate_string("abcde".to_string(), "cdeab".to_string()));
    assert!(!rotate_string("abcde".to_string(), "abced".to_string()));
    assert!(rotate_string(
        "defdefdefabcabc".to_string(),
        "defdefabcabcdef".to_string()
    ));
}
