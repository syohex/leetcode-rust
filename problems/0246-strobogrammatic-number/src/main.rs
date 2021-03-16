fn is_strobogrammatic(num: String) -> bool {
    let mut cs: Vec<char> = vec![' '; num.len()];
    for (i, c) in num.chars().enumerate() {
        if let Some(r) = match c {
            '0' | '1' | '8' => Some(c),
            '6' => Some('9'),
            '9' => Some('6'),
            _ => None,
        } {
            let index = num.len() - 1 - i;
            cs[index] = r;
        } else {
            return false;
        }
    }

    let str: String = cs.iter().collect();
    num == str
}

fn main() {
    println!("ret={}", is_strobogrammatic("111".to_string()));
}

#[test]
fn test_is_strobogrammatic() {
    assert!(is_strobogrammatic("69".to_string()));
    assert!(is_strobogrammatic("88".to_string()));
    assert!(!is_strobogrammatic("962".to_string()));
    assert!(is_strobogrammatic("1".to_string()));

    assert!(is_strobogrammatic("010".to_string()));
}
