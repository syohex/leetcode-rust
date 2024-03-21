fn is_substring_present(s: String) -> bool {
    let cs1: Vec<char> = s.chars().collect();
    let cs2: Vec<char> = s.chars().rev().collect();

    let cs1: Vec<_> = cs1.windows(2).collect();
    let cs2: Vec<_> = cs2.windows(2).collect();

    for &s1 in &cs1 {
        for &s2 in &cs2 {
            if *s1 == *s2 {
                return true;
            }
        }
    }

    false
}

fn main() {
    println!("ret={}", is_substring_present("leetcode".to_string()));
}

#[test]
fn test() {
    assert!(is_substring_present("leetcode".to_string()));
    assert!(is_substring_present("abcba".to_string()));
    assert!(!is_substring_present("abcd".to_string()));
}
