fn has_all_codes(s: String, k: i32) -> bool {
    use std::collections::HashSet;

    let mut hs: HashSet<&str> = HashSet::new();
    let limit = 1 << k;
    let k = k as usize;
    for i in k - 1..s.len() {
        let start = 1 + i - k;
        let substr = &s[start..i + 1];
        if !hs.contains(substr) {
            hs.insert(substr);
            if hs.len() >= limit {
                return true;
            }
        }
    }

    false
}

fn main() {
    let ret = has_all_codes("00110110".to_string(), 2);
    println!("ret={}", ret);
}

#[test]
fn test_has_all_codes() {
    assert!(has_all_codes("00110110".to_string(), 2));
    assert!(has_all_codes("00110".to_string(), 2));
    assert!(has_all_codes("0110".to_string(), 1));
    assert!(!has_all_codes("0110".to_string(), 2));
    assert!(!has_all_codes("0000000001011100".to_string(), 4));
    assert!(!has_all_codes("0110".to_string(), 20));
}
