fn has_all_codes(s: String, k: i32) -> bool {
    use std::collections::HashSet;

    let len = s.len();
    let k = k as usize;
    if k > len {
        return false;
    }

    let mut substrs = HashSet::new();
    for i in 0..=(len - k) {
        let substr = &s[i..i + k];
        substrs.insert(substr);
    }

    substrs.len() == 2usize.pow(k as u32)
}

fn main() {
    let ret = has_all_codes("00110110".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(has_all_codes("0".to_string(), 20));
    assert!(has_all_codes("00110110".to_string(), 2));
    assert!(has_all_codes("0110".to_string(), 1));
    assert!(!has_all_codes("0110".to_string(), 2));
}
