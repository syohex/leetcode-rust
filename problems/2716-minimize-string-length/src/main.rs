fn minimized_string_length(s: String) -> i32 {
    use std::collections::HashSet;
    s.chars().collect::<HashSet<char>>().len() as i32
}

fn main() {
    let ret = minimized_string_length("aaabc".to_string());
    println!("ret={ret}");
}

#[test]
fn test_minimized_string_length() {
    assert_eq!(minimized_string_length("aaabc".to_string()), 3);
    assert_eq!(minimized_string_length("cbbd".to_string()), 3);
    assert_eq!(minimized_string_length("dddaaa".to_string()), 2);
}
