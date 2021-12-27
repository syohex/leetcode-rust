fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|s| s.chars().filter(|c| *c == ' ').count() + 1)
        .max()
        .unwrap() as i32
}

fn main() {
    let sentences = vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ];
    println!("ret={}", most_words_found(sentences));
}

#[test]
fn test_most_words_found() {
    {
        let sentences = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(most_words_found(sentences), 6);
    }
    {
        let sentences = vec![
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string(),
        ];
        assert_eq!(most_words_found(sentences), 3);
    }
}
