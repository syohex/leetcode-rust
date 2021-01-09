use std::collections::HashSet;

fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let s: HashSet<char> = allowed.chars().collect();

    words
        .iter()
        .filter(|word| word.chars().filter(|c| !s.contains(&c)).count() == 0)
        .count() as i32
}

fn main() {
    let allowed = "ab".to_string();
    let words = vec![
        "ad".to_string(),
        "bd".into(),
        "aaab".into(),
        "baa".into(),
        "badab".into(),
    ];

    println!("example1 = {}", count_consistent_strings(allowed, words));
}

#[test]
fn test_count_consistent_strings() {
    {
        let allowed = "ab".to_string();
        let words = vec![
            "ad".to_string(),
            "bd".into(),
            "aaab".into(),
            "baa".into(),
            "badab".into(),
        ];
        assert_eq!(count_consistent_strings(allowed, words), 2);
    }
    {
        let allowed = "abc".to_string();
        let words = vec![
            "a".to_string(),
            "b".into(),
            "c".into(),
            "ab".into(),
            "ac".into(),
            "bc".into(),
            "abc".into(),
        ];
        assert_eq!(count_consistent_strings(allowed, words), 7);
    }
    {
        let allowed = "cad".to_string();
        let words = vec![
            "cc".to_string(),
            "acd".into(),
            "b".into(),
            "ba".into(),
            "bac".into(),
            "bad".into(),
            "ac".into(),
            "d".into(),
        ];
        assert_eq!(count_consistent_strings(allowed, words), 4);
    }
}
