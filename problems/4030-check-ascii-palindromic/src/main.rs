fn is_palindromic(s: String) -> bool {
    let mut t = String::new();
    for b in s.bytes() {
        let bs = format!("{b:08b}");
        t.push_str(&bs);
    }

    let rev: String = t.chars().rev().collect();
    t == rev
}

fn main() {
    let ret = is_palindromic("ff".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_palindromic("ff".to_string()));
    assert!(!is_palindromic("leet".to_string()));
}
