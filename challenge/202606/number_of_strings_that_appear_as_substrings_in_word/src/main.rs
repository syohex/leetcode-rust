fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    patterns.into_iter().filter(|p| word.contains(p)).count() as i32
}

fn main() {
    let patterns = vec![
        "a".to_string(),
        "abc".to_string(),
        "bc".to_string(),
        "d".to_string(),
    ];
    let word = "abc".to_string();
    let ret = num_of_strings(patterns, word);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let patterns = vec![
            "a".to_string(),
            "abc".to_string(),
            "bc".to_string(),
            "d".to_string(),
        ];
        let word = "abc".to_string();
        let ret = num_of_strings(patterns, word);
        assert_eq!(ret, 3);
    }
    {
        let patterns = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let word = "aaaaaaaaaabbbbbbbbbbbbb".to_string();
        let ret = num_of_strings(patterns, word);
        assert_eq!(ret, 2);
    }
}
