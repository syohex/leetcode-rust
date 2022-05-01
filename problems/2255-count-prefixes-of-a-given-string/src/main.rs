fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    words.into_iter().filter(|word| s.starts_with(word)).count() as i32
}

fn main() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "ab".to_string(),
        "bc".to_string(),
        "abc".to_string(),
    ];
    let s = "abc".to_string();
    let ret = count_prefixes(words, s);
    println!("ret={ret}");
}

#[test]
fn test_count_prefixes() {
    {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        let s = "abc".to_string();
        let ret = count_prefixes(words, s);
        assert_eq!(ret, 3);
    }
    {
        let words = vec!["a".to_string(), "a".to_string()];
        let s = "aa".to_string();
        let ret = count_prefixes(words, s);
        assert_eq!(ret, 2);
    }
}
