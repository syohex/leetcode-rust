fn is_acronym(words: Vec<String>, s: String) -> bool {
    let acronym = words.into_iter().fold(String::new(), |mut acc, word| {
        acc.push(word.chars().next().unwrap());
        acc
    });

    acronym == s
}

fn main() {
    let ret = is_acronym(
        vec![
            "never".to_string(),
            "gonna".to_string(),
            "give".to_string(),
            "up".to_string(),
            "on".to_string(),
            "you".to_string(),
        ],
        "ngguoy".to_string(),
    );
    println!("ret={ret}");
}

#[test]
fn test_is_acronym() {
    assert!(is_acronym(
        vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string()
        ],
        "abc".to_string()
    ));
    assert!(!is_acronym(
        vec!["an".to_string(), "apple".to_string()],
        "a".to_string()
    ));
    assert!(is_acronym(
        vec![
            "never".to_string(),
            "gonna".to_string(),
            "give".to_string(),
            "up".to_string(),
            "on".to_string(),
            "you".to_string()
        ],
        "ngguoy".to_string()
    ));
}
