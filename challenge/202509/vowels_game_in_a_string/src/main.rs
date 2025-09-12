fn does_alice_win(s: String) -> bool {
    s.contains(['a', 'e', 'i', 'o', 'u'])
}

fn main() {
    let ret = does_alice_win("leetcoder".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(does_alice_win("leetcoder".to_string()));
    assert!(!does_alice_win("bbcd".to_string()));
}
