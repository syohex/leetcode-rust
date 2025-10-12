fn score_balance(s: String) -> bool {
    let mut right = s.bytes().fold(0, |acc, b| acc + (b - b'a' + 1) as i32);

    let mut left = 0;
    for b in s.bytes() {
        let v = (b - b'a' + 1) as i32;
        left += v as i32;
        right -= v as i32;
        dbg!(left, right);
        if left == right {
            return true;
        }
    }

    false
}

fn main() {
    let ret = score_balance(String::from("abdcd"));
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(score_balance("abdcd".to_string()));
    assert!(score_balance("adcb".to_string()));
    assert!(!score_balance("bace".to_string()));
}
