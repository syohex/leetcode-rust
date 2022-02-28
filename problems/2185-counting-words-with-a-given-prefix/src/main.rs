fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words
        .into_iter()
        .filter(|word| word.starts_with(&pref))
        .count() as i32
}

fn main() {
    let words = vec![
        "pay".to_string(),
        "attention".to_string(),
        "practice".to_string(),
        "attent".to_string(),
    ];
    let pref = "at".to_string();
    let ret = prefix_count(words, pref);
    println!("ret={ret}");
}

#[test]
fn test_prefix_count() {
    {
        let words = vec![
            "pay".to_string(),
            "attention".to_string(),
            "practice".to_string(),
            "attent".to_string(),
        ];
        let pref = "at".to_string();
        assert_eq!(prefix_count(words, pref), 2);
    }
    {
        let words = vec![
            "leetcode".to_string(),
            "win".to_string(),
            "loops".to_string(),
            "success".to_string(),
        ];
        let pref = "code".to_string();
        assert_eq!(prefix_count(words, pref), 0);
    }
}
