fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s.contains(x))
        .map(|d| d.0 as i32)
        .collect()
}

fn main() {
    let words = vec!["leet".to_string(), "code".to_string()];
    let ret = find_words_containing(words, 'e');
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec!["leet".to_string(), "code".to_string()];
        let expected = vec![0, 1];
        let ret = find_words_containing(words, 'e');
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let expected = vec![0, 2];
        let ret = find_words_containing(words, 'a');
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let expected = vec![];
        let ret = find_words_containing(words, 'e');
        assert_eq!(ret, expected);
    }
}
