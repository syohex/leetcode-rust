fn check_ones_segment(s: String) -> bool {
    !s.contains("01")
}

fn main() {
    println!(
        "check_ones_segment({})={}",
        "1001",
        check_ones_segment("1001".to_string())
    );
}

#[test]
fn test_check_ones_segment() {
    assert!(!check_ones_segment("1001".to_string()));
    assert!(check_ones_segment("110".to_string()));
    assert!(check_ones_segment("1".to_string()));
    assert!(check_ones_segment("10".to_string()));
}
