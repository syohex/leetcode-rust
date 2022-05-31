fn has_all_codes(s: String, k: i32) -> bool {
    use std::collections::HashSet;

    let k = k as usize;
    if s.len() < k {
        return false;
    }

    let mut checked = HashSet::new();
    for i in 0..=(s.len() - k) {
        let substr = s[i..i + k].to_string();
        checked.insert(substr);
    }

    checked.len() == 2usize.pow(k as u32)
}
fn main() {
    let s = "00110110".to_string();
    let ret = has_all_codes(s, 2);
    println!("ret={ret}");
}

#[test]
fn test_has_all_codes() {
    {
        let s = "00110110".to_string();
        let ret = has_all_codes(s, 2);
        assert!(ret);
    }
    {
        let s = "0110".to_string();
        let ret = has_all_codes(s, 1);
        assert!(ret);
    }
    {
        let s = "0110".to_string();
        let ret = has_all_codes(s, 2);
        assert!(!ret);
    }
    {
        let s = "0101".to_string();
        let ret = has_all_codes(s, 13);
        assert!(!ret);
    }
}
