fn password_strength(password: String) -> i32 {
    use std::collections::HashSet;

    let mut lowers = HashSet::new();
    let mut uppers = HashSet::new();
    let mut digits = HashSet::new();
    let mut specials = HashSet::new();

    for c in password.chars() {
        if c.is_ascii_lowercase() {
            lowers.insert(c);
        } else if c.is_ascii_uppercase() {
            uppers.insert(c);
        } else if c.is_ascii_digit() {
            digits.insert(c);
        } else {
            specials.insert(c);
        }
    }

    (lowers.len() + 2 * uppers.len() + 3 * digits.len() + 5 * specials.len()) as i32
}

fn main() {
    let ret = password_strength("aA1!".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(password_strength("aA1!".to_string()), 11);
    assert_eq!(password_strength("bbB11#".to_string()), 11);
}
