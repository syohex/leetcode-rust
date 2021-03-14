fn are_almost_equal(s1: String, s2: String) -> bool {
    use std::collections::HashSet;

    let h1: HashSet<char> = s1.chars().collect();
    let h2: HashSet<char> = s2.chars().collect();

    let not_matched = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| *c1 != *c2)
        .count();

    h1 == h2 && (not_matched == 0 || not_matched == 2)
}

fn main() {
    let ret = are_almost_equal("bank".to_string(), "kanb".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_are_almost_equal() {
    assert!(are_almost_equal("bank".to_string(), "kanb".to_string()));
    assert!(!are_almost_equal(
        "attack".to_string(),
        "defend".to_string()
    ));
    assert!(are_almost_equal("kelb".to_string(), "kelb".to_string()));
    assert!(!are_almost_equal("abcd".to_string(), "dcba".to_string()));
    assert!(!are_almost_equal("aa".to_string(), "bb".to_string()));
}
