fn is_anagram(s: String, t: String) -> bool {
    let mut sv: Vec<char> = s.chars().collect();
    let mut tv: Vec<char> = t.chars().collect();
    sv.sort_unstable();
    tv.sort_unstable();

    sv == tv
}

fn main() {
    println!(
        "is_anagram('anagram', 'nagram')={}",
        is_anagram("anagram".to_string(), "nagram".to_string())
    );
}

#[test]
fn test_is_anagram() {
    assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!is_anagram("rat".to_string(), "car".to_string()));
}
